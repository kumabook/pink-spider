use std::io::Read;
use hyper::header::Connection;
use rustc_serialize::json;
use rustc_serialize::json::{DecodeResult};
use get_env;
use http;

static BASE_URL: &'static str = "https://api.soundcloud.com";

pub static TRACK:    &'static str = r"api.soundcloud.com/tracks/([a-zA-Z0-9_-]+)";
pub static PLAYLIST: &'static str = r"api.soundcloud.com/playlists/([a-zA-Z0-9_-]+)";
pub static USER:     &'static str = r"api.soundcloud.com/users/([a-zA-Z0-9_-]+)";

lazy_static! {
    static ref API_KEY: String = {
        get_env::var("SOUNDCLOUD_API_KEY").unwrap_or("".to_string())
    };
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Playlist {
    pub id:            i32,
    pub created_at:    String,
    pub user_id:       i32,
    pub user:          User,
    pub title:         String,
    pub permalink:     String,
    pub permalink_url: String,
    pub artwork_url:   Option<String>,
    pub tracks:        Vec<Track>,
    pub genre:         Option<String>,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Track {
    pub id:            i32,
    pub created_at:    String,
    pub title:         String,
    pub description:   String,
    pub user_id:       i32,
    pub user:          User,
    pub permalink:     String,
    pub permalink_url: String,
    pub uri:           String,
    pub artwork_url:   Option<String>,
    pub duration:      i32,
    pub stream_url:    String,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct User {
    pub id:            i32,
    pub username:      String,
    pub permalink:     String,
    pub uri:           String,
    pub permalink_url: String,
    pub avatar_url:    String,
}

pub fn fetch_track(id: &str) -> DecodeResult<Track> {
    let params = format!("client_id={}", *API_KEY);
    let url    = format!("{}/tracks/{}?{}", BASE_URL, id, params);
    let mut res = http::client().get(&url)
                                .header(Connection::close())
                                .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    return json::decode::<Track>(&body)
}

pub fn fetch_playlist(id: &str) -> DecodeResult<Playlist> {
    let params = format!("client_id={}", *API_KEY);
    let url    = format!("{}/playlists/{}?{}", BASE_URL, id, params);
    let mut res = http::client().get(&url)
                                .header(Connection::close())
                                .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    return  json::decode::<Playlist>(&body)
}


pub fn fetch_user_tracks(id: &str) -> DecodeResult<Vec<Track>> {
    let params = format!("client_id={}", *API_KEY);
    let url    = format!("{}/users/{}/tracks?{}", BASE_URL, id, params);
    let mut res = http::client().get(&url)
                                .header(Connection::close())
                                .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    return  json::decode::<Vec<Track>>(&body)
}

