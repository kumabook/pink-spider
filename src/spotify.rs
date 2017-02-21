extern crate rustc_serialize;

use std::io::Read;
use std::env;
use std::collections::BTreeMap;
use std::fs::File;
use std::sync::Mutex;
use chrono::{NaiveDateTime, UTC, Duration};
use rustc_serialize::json;
use hyper::Client;
use hyper::header::{
    Headers,
    Authorization,
    Bearer,
    Basic,
    Connection,
    ContentType
};

static BASE_URL: &'static str = "https://api.spotify.com/v1";
lazy_static! {
    static ref CLIENT_ID: String = {
        let opt_key = env::var("SPOTIFY_CLIENT_ID");
        match opt_key {
            Ok(key) => key,
            Err(_) => {
                let mut f = File::open("spotify_client_id.txt").unwrap();
                let mut s = String::new();
                let _ = f.read_to_string(&mut s);
                s
            }
        }
    };
    static ref CLIENT_SECRET: String = {
        let opt_key = env::var("SPOTIFY_CLIENT_SECRET");
        match opt_key {
            Ok(key) => key,
            Err(_) => {
                let mut f = File::open("spotify_client_secret.txt").unwrap();
                let mut s = String::new();
                let _ = f.read_to_string(&mut s);
                s
            }
        }
    };
    static ref TOKEN: Mutex<Option<Token>> = Mutex::new(None);
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Track {
    pub album:             Album,
    pub artists:           Vec<Artist>,
    pub available_markets: Vec<String>,
    pub disc_number:       i32,
    pub duration_ms:       i32,
    pub explicit:          bool,
    pub external_ids:      BTreeMap<String, String>,
    pub external_urls:     BTreeMap<String, String>,
    pub href:              String,
    pub id:                String,
    pub is_playable:       Option<bool>,
    pub linked_from:       Option<TrackLink>,
    pub name:              String,
    pub popularity:        i32,
    pub preview_url:       Option<String>,
    pub track_number:      i32,
//  pub type:            String, TODO Use serde instead of rustc_serialize
    pub uri:               String,
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Playlist {
    pub collaborative: bool,
    pub description:   Option<String>,
    pub external_urls: BTreeMap<String, String>,
    pub followers:     Followers,
    pub href:          Option<String>,
    pub id:            String,
    pub images:        Vec<Image>,
    pub name:          String,
    pub owner:         User,
    pub public:        Option<bool>,
    pub snapshot_id:   String,
    pub tracks:        PagingObject<PlaylistTrack>,
//  pub type           String, TODO Use serde instead of rustc_serialize
    pub uri:           String,
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct User {
    pub display_name:  Option<String>,
    pub external_urls: BTreeMap<String, String>,
    pub followers:     Option<Followers>,
    pub href:          String,
    pub id:            String,
    pub images:        Option<Vec<Image>>,
    pub uri:           String,
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Album {
    pub album_type: String,
    pub artists:    Vec<Artist>,
    pub available_markets: Vec<String>,
    pub external_urls: BTreeMap<String, String>,
    pub href:          String,
    pub id:            String,
    pub images:        Vec<Image>,
    pub name:          String,
//  pub type:            String, TODO Use serde instead of rustc_serialize
    pub uri:           String,
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Artist {
    pub external_urls: BTreeMap<String, String>,
    pub href:          String,
    pub id:            String,
    pub name:          String,
//  pub type:            String, TODO Use serde instead of rustc_serialize
    pub uri:           String,
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Image {
    pub height: Option<i32>,
    pub url:    String,
    pub width:  Option<i32>,
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct TrackLink {
    pub external_urls: BTreeMap<String, String>,
    pub href:          String,
    pub id:            String,
//  pub type:            String, TODO Use serde instead of rustc_serialize
    pub uri:           String,
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Followers {
    pub href:  Option<String>,
    pub total: i32,
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct PlaylistTrack {
    pub added_at: String,
    pub added_by: User,
    pub is_local: bool,
    pub track:    Track,
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct PagingObject<T> {
    pub href:     String,
    pub items:    Vec<T>,
    pub limit:    i32,
    pub next:     Option<String>,
    pub offset:   i32,
    pub previous: Option<String>,
    pub total:    i32,
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Token {
    pub access_token: String,
    pub token_type:   String,
    pub expires_in:   i64,
    pub expires_at:   Option<NaiveDateTime>,
}

/// This function fetches a track info with spotify api.
///
/// # Examples
///
/// ```
/// let track = pink_spider::spotify::fetch_track("3n3Ppam7vgaVa1iaRUc9Lp").unwrap();
///
/// assert_eq!(track.id, "3n3Ppam7vgaVa1iaRUc9Lp");
/// ```
pub fn fetch_track(id: &str) -> json::DecodeResult<Track> {
    let path = format!("/tracks/{}", id);
    fetch(&path)
}

/// This function fetches a playlist info with spotify api.
///
/// # Examples
///
/// ```
/// let playlist = pink_spider::spotify::fetch_playlist("spincoaster", "182jSXyIDGLOYwE7PLhxjI").unwrap();
///
/// assert_eq!(playlist.id, "182jSXyIDGLOYwE7PLhxjI");
/// assert_eq!(playlist.tracks.total, 100);
/// ```
pub fn fetch_playlist(user_id: &str, id: &str) -> json::DecodeResult<Playlist> {
    let path = format!("/users/{}/playlists/{}", user_id, id);
    fetch(&path)
}

fn fetch<T>(path: &str) -> json::DecodeResult<T>
    where T: rustc_serialize::Decodable {
    let token  = try!(update_token_if_needed());
    let url    = format!("{}{}", BASE_URL, path);
    let client = Client::new();
    let mut headers = Headers::new();
    headers.set(
        Authorization(
            Bearer {
                token: token.access_token
            }
        )
    );
    headers.set(Connection::close());
    let mut res = client.get(&url)
                        .headers(headers)
                        .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    json::decode::<T>(&body)
}

/// This function fetches a oauth token info with spotify api.
///
/// # Examples
///
/// ```
/// let token = pink_spider::spotify::fetch_token();
/// assert!(token.is_ok());
/// ```
pub fn fetch_token() -> json::DecodeResult<Token> {
    let url         = "https://accounts.spotify.com/api/token";
    let client      = Client::new();
    let mut headers = Headers::new();
    headers.set(
        Authorization(
            Basic {
                username: CLIENT_ID.to_string(),
                password: Some(CLIENT_SECRET.to_string()),
            }
        )
    );
    headers.set(ContentType("application/x-www-form-urlencoded".parse().unwrap()));
    headers.set(Connection::close());
    let mut res = client.post(url)
                        .body("grant_type=client_credentials")
                        .headers(headers)
                        .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    json::decode::<Token>(&body)
}

pub fn get_valid_token() -> Option<Token> {
    TOKEN.lock().unwrap().clone().and_then(
        |t|
        if let Some(expires_at) = t.expires_at {
            if expires_at > UTC::now().naive_utc() {
                Some(t)
            } else {
                None
            }
        } else {
            None
        }
    )
}

pub fn update_token_if_needed() -> json::DecodeResult<Token> {
    match get_valid_token() {
        Some(token) => Ok(token),
        None => match fetch_token() {
            Ok(token) => {
                let mut t        = TOKEN.lock().unwrap();
                let mut token    = token.clone();
                let now          = UTC::now().naive_utc();
                let exires_at    = now + Duration::seconds(token.expires_in);
                token.expires_at = Some(exires_at);
                *t = Some(token.clone());
                Ok(token)
            },
            Err(_) =>
                Err(json::DecoderError::ApplicationError("missing token".to_string()))
        },
    }
}
