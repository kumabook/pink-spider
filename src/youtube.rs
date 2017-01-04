use std::io::Read;
use std::env;
use hyper::Client;
use hyper::header::Connection;
use std::collections::BTreeMap;
use rustc_serialize::json;
use std::fs::File;

static BASE_URL:    &'static str = "https://www.googleapis.com/youtube/v3";
lazy_static! {
    static ref API_KEY: String = {
        let opt_key = env::var("YOUTUBE_API_KEY");
        match opt_key {
            Ok(key) => key,
            Err(_) => {
                let mut f = File::open("youtube.txt").unwrap();
                let mut s = String::new();
                let _ = f.read_to_string(&mut s);
                s
            }
        }
    };
}

pub trait HasThumbnail {
    fn get_thumbnails(&self) -> &BTreeMap<String, Thumbnail>;
    fn get_thumbnail_url(&self) -> Option<String> {
        self.get_thumbnails().get(        "default" ).map(|t| t.url.to_string())
            .or(self.get_thumbnails().get("medium"  ).map(|t| t.url.to_string()))
            .or(self.get_thumbnails().get("high"    ).map(|t| t.url.to_string()))
            .or(self.get_thumbnails().get("standard").map(|t| t.url.to_string()))
            .or(self.get_thumbnails().get("maxres"  ).map(|t| t.url.to_string()))
    }
    fn get_artwork_url(&self) -> Option<String> {
        self.get_thumbnails().get(        "maxres"  ).map(|t| t.url.to_string())
            .or(self.get_thumbnails().get("standard").map(|t| t.url.to_string()))
            .or(self.get_thumbnails().get("high"    ).map(|t| t.url.to_string()))
            .or(self.get_thumbnails().get("medium"  ).map(|t| t.url.to_string()))
            .or(self.get_thumbnails().get("default" ).map(|t| t.url.to_string()))
    }
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
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
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct PlaylistItemResponse {
    pub kind:          String,
    pub etag:          String,
    pub nextPageToken: String,
    pub pageInfo:      BTreeMap<String, i32>,
    pub items:         Vec<PlaylistItem>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct PlaylistItem {
    pub kind:    String,
    pub etag:    String,
    pub id:      String,
    pub snippet: PlaylistItemSnippet,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct PlaylistItemSnippet {
    pub title:         String,
    pub description:   String,
    pub publishedAt:   String,
    pub channelId:     String,
    pub channelTitle:  String,
    pub thumbnails:    BTreeMap<String, Thumbnail>,
    pub position:      i32,
    pub playlistId:    String,
    pub resourceId:    BTreeMap<String, String>
}

#[allow(non_snake_case)]
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct VideoResponse {
    pub kind:          String,
    pub etag:          String,
    pub pageInfo:      BTreeMap<String, i32>,
    pub items:         Vec<Video>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Video {
    pub kind:    String,
    pub etag:    String,
    pub id:      String,
    pub snippet: VideoSnippet,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct VideoSnippet {
    pub title:                String,
    pub description:          String,
    pub publishedAt:          String,
    pub channelId:            String,
    pub channelTitle:         String,
    pub thumbnails:           BTreeMap<String, Thumbnail>,
    pub tags:                 Option<Vec<String>>,
    pub categoryId:           String,
    pub liveBroadcastContent: String,
}

impl HasThumbnail for PlaylistItemSnippet {
    fn get_thumbnails(&self) -> &BTreeMap<String, Thumbnail> {
        &self.thumbnails
    }
}

impl HasThumbnail for VideoSnippet {
    fn get_thumbnails(&self) -> &BTreeMap<String, Thumbnail> {
        &self.thumbnails
    }
}

pub fn fetch_playlist(id: &str) -> json::DecodeResult<PlaylistItemResponse> {
    let params = format!("key={}&part=snippet&playlistId={}", *API_KEY, id);
    let url    = format!("{}/{}?{}", BASE_URL, "playlistItems", params);
    let client = Client::new();
    let mut res = client.get(&url)
                        .header(Connection::close())
                        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    json::decode::<PlaylistItemResponse>(&body)
}

pub fn fetch_video(id: &str) -> json::DecodeResult<Video> {
    let params = format!("key={}&part=snippet&id={}", *API_KEY, id);
    let url    = format!("{}/{}?{}", BASE_URL, "videos", params);
    let client = Client::new();
    let mut res = client.get(&url)
                        .header(Connection::close())
                        .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let res = try!(json::decode::<VideoResponse>(&body));
    if res.items.len() > 0 {
        return Ok(res.items[0].clone());
    }
    Err(json::DecoderError::ApplicationError("track not found".to_string()))
}
