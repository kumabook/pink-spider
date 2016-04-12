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


#[derive(Debug, RustcDecodable, RustcEncodable)]
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

pub fn fetch_playlist(id: &str) -> json::DecodeResult<PlaylistItemResponse> {
    let params = format!("key={}&part=snippet&playlistId={}", *API_KEY, id);
    let url    = format!("{}/{}?{}",
                         BASE_URL,
                         "playlistItems",
                         params);
    println!("{}", url);
    let client = Client::new();
    let mut res = client.get(&url)
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    return  json::decode::<PlaylistItemResponse>(&body)
}

