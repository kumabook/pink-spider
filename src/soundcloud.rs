use std::io::Read;
use std::env;
use hyper::Client;
use hyper::header::Connection;
use rustc_serialize::json;
use rustc_serialize::json::{DecodeResult};
use std::fs::File;

static BASE_URL: &'static str = "https://api.soundcloud.com";
lazy_static! {
    static ref API_KEY: String = {
        let opt_key = env::var("SOUNDCLOUD_API_KEY");
        match opt_key {
            Ok(key) => key,
            Err(_) => {
                let mut f = File::open("soundcloud.txt").unwrap();
                let mut s = String::new();
                let _ = f.read_to_string(&mut s);
                s
            }
        }
    };
}

#[allow(non_snake_case)]
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Playlist {
    pub id:            i32,
    pub created_at:    String,
    pub user_id:       i32,
    pub title:         String,
    pub permalink:     String,
    pub permalink_url: String,
    pub artwork_url:   Option<String>,
    pub tracks:        Vec<Track>,
}

#[allow(non_snake_case)]
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Track {
    pub id:            i32,
    pub created_at:    String,
    pub user_id:       i32,
    pub title:         String,
    pub permalink:     String,
    pub permalink_url: String,
    pub uri:           String,
    pub artwork_url:   Option<String>,
    pub description:   String,
    pub duration:      i32,
    pub stream_url:    String,
}


pub fn fetch_playlist(id: &str) -> DecodeResult<Playlist> {
    let params = format!("client_id={}", *API_KEY);
    let url    = format!("{}/playlists/{}?{}", BASE_URL, id, params);
    println!("{}", url);
    let client = Client::new();
    let mut res = client.get(&url)
                        .header(Connection::close())
                        .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    return  json::decode::<Playlist>(&body)
}


pub fn fetch_user_tracks(id: &str) -> DecodeResult<Vec<Track>> {
    let params = format!("client_id={}", *API_KEY);
    let url    = format!("{}/users/{}/tracks?{}", BASE_URL, id, params);
    println!("{}", url);
    let client = Client::new();
    let mut res = client.get(&url)
                        .header(Connection::close())
                        .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    return  json::decode::<Vec<Track>>(&body)
}

