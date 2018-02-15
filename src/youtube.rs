use std::io::Read;
use hyper::header::Connection;
use std::collections::BTreeMap;
use serde_json;
use serde::de::Error;

use get_env;
use http;

static BASE_URL:    &'static str = "https://www.googleapis.com/youtube/v3";
static MAX_RESULTS: i32          = 50;
lazy_static! {
    static ref API_KEY: String = {
        get_env::var("YOUTUBE_API_KEY").unwrap_or("".to_string())
    };
}

pub static EMBED: &'static str = r"www.youtube.com/embed/([a-zA-Z0-9_-].+)";
pub static LIST:  &'static str = r"www.youtube.com/embed/videoseries\?list=([a-zA-Z0-9_-]+)";
pub static WATCH: &'static str = r"www.youtube.com/watch\?v=([a-zA-Z0-9_-]+)";

pub trait HasThumbnail {
    fn get_thumbnails(&self) -> BTreeMap<String, Thumbnail>;
    fn get_thumbnail_url(&self) -> Option<String> {
        let thumbs = self.get_thumbnails();
        thumbs.get(         "default").map(|t| t.url.to_string())
            .or(thumbs.get("medium"  ).map(|t| t.url.to_string()))
            .or(thumbs.get("high"    ).map(|t| t.url.to_string()))
            .or(thumbs.get("standard").map(|t| t.url.to_string()))
            .or(thumbs.get("maxres"  ).map(|t| t.url.to_string()))
    }
    fn get_artwork_url(&self) -> Option<String> {
        let thumbs = self.get_thumbnails();
        thumbs.get(          "maxres").map(|t| t.url.to_string())
            .or(thumbs.get("standard").map(|t| t.url.to_string()))
            .or(thumbs.get("high"    ).map(|t| t.url.to_string()))
            .or(thumbs.get("medium"  ).map(|t| t.url.to_string()))
            .or(thumbs.get("default" ).map(|t| t.url.to_string()))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thumbnail {
    pub url:    String,
    pub width:  i32,
    pub height: i32
}

impl PartialEq for Thumbnail {
    fn eq(&self, t: &Thumbnail) -> bool {
        return self.url == t.url;
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistResponse {
    pub kind:          String,
    pub etag:          String,
    pub nextPageToken: Option<String>,
    pub pageInfo:      BTreeMap<String, i32>,
    pub items:         Vec<Playlist>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Playlist {
    pub kind:    String,
    pub etag:    String,
    pub id:      String,
    pub snippet: PlaylistSnippet,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistSnippet {
    pub title:         String,
    pub description:   String,
    pub publishedAt:   String,
    pub channelId:     String,
    pub channelTitle:  String,
    pub thumbnails:    Option<BTreeMap<String, Thumbnail>>,
    pub tags:          Option<Vec<String>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistItemResponse {
    pub kind:          String,
    pub etag:          String,
    pub nextPageToken: Option<String>,
    pub pageInfo:      BTreeMap<String, i32>,
    pub items:         Vec<PlaylistItem>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistItem {
    pub kind:    String,
    pub etag:    String,
    pub id:      String,
    pub snippet: PlaylistItemSnippet,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistItemSnippet {
    pub title:         String,
    pub description:   String,
    pub publishedAt:   String,
    pub channelId:     String,
    pub channelTitle:  String,
    pub thumbnails:    Option<BTreeMap<String, Thumbnail>>,
    pub position:      i32,
    pub playlistId:    String,
    pub resourceId:    BTreeMap<String, String>
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct VideoResponse {
    pub kind:          String,
    pub etag:          String,
    pub pageInfo:      BTreeMap<String, i32>,
    pub items:         Vec<Video>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Video {
    pub kind:    String,
    pub etag:    String,
    pub id:      String,
    pub snippet: VideoSnippet,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoSnippet {
    pub title:                String,
    pub description:          String,
    pub publishedAt:          String,
    pub channelId:            String,
    pub channelTitle:         String,
    pub thumbnails:           Option<BTreeMap<String, Thumbnail>>,
    pub tags:                 Option<Vec<String>>,
    pub categoryId:           String,
    pub liveBroadcastContent: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelResponse {
    pub kind:          String,
    pub etag:          String,
    pub pageInfo:      BTreeMap<String, i32>,
    pub items:         Vec<Channel>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channel {
    pub kind:    String,
    pub etag:    String,
    pub id:      String,
    pub snippet: ChannelSnippet,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelSnippet {
    pub title:                String,
    pub description:          String,
    pub publishedAt:          String,
    pub thumbnails:           Option<BTreeMap<String, Thumbnail>>,
}

impl HasThumbnail for PlaylistSnippet {
    fn get_thumbnails(&self) -> BTreeMap<String, Thumbnail> {
        self.thumbnails.clone().unwrap_or(BTreeMap::new())
    }
}

impl HasThumbnail for PlaylistItemSnippet {
    fn get_thumbnails(&self) -> BTreeMap<String, Thumbnail> {
        self.thumbnails.clone().unwrap_or(BTreeMap::new())
    }
}

impl HasThumbnail for VideoSnippet {
    fn get_thumbnails(&self) -> BTreeMap<String, Thumbnail> {
        self.thumbnails.clone().unwrap_or(BTreeMap::new())
    }
}

impl HasThumbnail for ChannelSnippet {
    fn get_thumbnails(&self) -> BTreeMap<String, Thumbnail> {
        self.thumbnails.clone().unwrap_or(BTreeMap::new())
    }
}

pub fn fetch_playlist(id: &str) -> serde_json::Result<PlaylistResponse> {
    let params = format!("key={}&part=snippet&id={}&maxResults={}",
                         *API_KEY,
                         id,
                         MAX_RESULTS);
    let url     = format!("{}/{}?{}", BASE_URL, "playlists", params);
    let mut res = http::client().get(&url)
                                .header(Connection::close())
                                .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    serde_json::from_str(&body)
}

pub fn fetch_playlist_items(id: &str) -> serde_json::Result<PlaylistItemResponse> {
    let params = format!("key={}&part=snippet&playlistId={}&maxResults={}",
                         *API_KEY,
                         id,
                         MAX_RESULTS);
    let url    = format!("{}/{}?{}", BASE_URL, "playlistItems", params);
    let mut res = http::client().get(&url)
                                .header(Connection::close())
                                .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    serde_json::from_str(&body)
}

pub fn fetch_video(id: &str) -> serde_json::Result<Video> {
    let params = format!("key={}&part=snippet&id={}", *API_KEY, id);
    let url    = format!("{}/{}?{}", BASE_URL, "videos", params);
    let mut res = http::client().get(&url)
                                .header(Connection::close())
                                .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let vr: VideoResponse = try!(serde_json::from_str(&body));
    if vr.items.len() > 0 {
        return Ok(vr.items[0].clone());
    }
    Err(serde_json::error::Error::custom("track not found".to_string()))
}

pub fn fetch_channel(id: &str) -> serde_json::Result<Channel> {
    let params = format!("key={}&part=snippet&id={}", *API_KEY, id);
    let url    = format!("{}/{}?{}", BASE_URL, "channels", params);
    let mut res = http::client().get(&url)
                                .header(Connection::close())
                                .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let vr: ChannelResponse = try!(serde_json::from_str(&body));
    if vr.items.len() > 0 {
        return Ok(vr.items[0].clone());
    }
    Err(serde_json::error::Error::custom("channel not found".to_string()))
}
