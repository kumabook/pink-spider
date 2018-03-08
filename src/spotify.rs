use std::io::Read;
use std::collections::BTreeMap;
use std::sync::Mutex;
use chrono::{NaiveDateTime, Utc, Duration};
use reqwest::header::{
    Headers,
    Authorization,
    Bearer,
    Basic,
    Connection,
    ContentType
};
use regex::Regex;
use serde_json;
use serde::de::Error;
use serde::de::DeserializeOwned;
use get_env;
use http;

static BASE_URL:           &'static str = "https://api.spotify.com/v1";
pub static TRACK_URI:      &'static str = r"spotify:track:([a-zA-Z0-9_-]+)";
pub static TRACK_OPEN:     &'static str = r"open.spotify.com/track/([a-zA-Z0-9_-]+)";
pub static TRACK_EMBED:    &'static str = r"open.spotify.com/embed/track/([a-zA-Z0-9_-]+)";
pub static PLAYLIST_URI:   &'static str = r"(spotify:user:([a-zA-Z0-9_-]+):playlist:([a-zA-Z0-9_-]+))";
pub static PLAYLIST_OPEN:  &'static str = r"(open.spotify.com/user/([a-zA-Z0-9_-]+)/playlist/([a-zA-Z0-9_-]+))";
pub static PLAYLIST_EMBED: &'static str = r"(open.spotify.com/embed/user/([a-zA-Z0-9_-]+)/playlist/([a-zA-Z0-9_-]+))";
pub static ALBUM_URI:      &'static str = r"spotify:album:([a-zA-Z0-9_-]+)";
pub static ALBUM_OPEN:     &'static str = r"open.spotify.com/album/([a-zA-Z0-9_-]+)";
pub static ALBUM_EMBED:    &'static str = r"open.spotify.com/embed/album/([a-zA-Z0-9_-]+)";

lazy_static! {
    static ref CLIENT_ID: String = {
        get_env::var("SPOTIFY_CLIENT_ID").unwrap_or("".to_string())
    };
    static ref CLIENT_SECRET: String = {
        get_env::var("SPOTIFY_CLIENT_SECRET").unwrap_or("".to_string())
    };
    static ref TOKEN: Mutex<Option<Token>> = Mutex::new(None);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Track {
    pub album:             Option<Album>,
    pub artists:           Vec<Artist>,
    pub available_markets: Vec<String>,
    pub disc_number:       i32,
    pub duration_ms:       i32,
    pub explicit:          bool,
    pub external_ids:      Option<BTreeMap<String, String>>,
    pub external_urls:     BTreeMap<String, String>,
    pub href:              String,
    pub id:                String,
    pub is_playable:       Option<bool>,
    pub linked_from:       Option<TrackLink>,
    pub name:              String,
    pub popularity:        Option<i32>,
    pub preview_url:       Option<String>,
    pub track_number:      i32,
//  pub type:            String, TODO Use serde instead of rustc_serialize
    pub uri:               String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Tracks {
    tracks: Vec<Track>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Playlist {
    pub collaborative: Option<bool>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub display_name:  Option<String>,
    pub external_urls: BTreeMap<String, String>,
    pub followers:     Option<Followers>,
    pub href:          String,
    pub id:            String,
    pub images:        Option<Vec<Image>>,
    pub uri:           String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Album {
    pub album_type:        String,
    pub artists:           Vec<Artist>,
    pub available_markets: Vec<String>,
    pub external_urls:     BTreeMap<String, String>,
    pub href:              String,
    pub id:                String,
    pub images:            Vec<Image>,
    pub name:              String,
//  pub type:              String, TODO Use serde instead of rustc_serialize
    pub uri:               String,
    pub tracks:            Option<PagingObject<Track>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Artist {
    pub external_urls: BTreeMap<String, String>,
    pub href:          String,
    pub id:            String,
    pub name:          String,
//  pub type:            String, TODO Use serde instead of rustc_serialize
    pub uri:           String,
    pub images:        Option<Vec<Image>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Artists {
    artists: Vec<Artist>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub height: Option<i32>,
    pub url:    String,
    pub width:  Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrackLink {
    pub external_urls: BTreeMap<String, String>,
    pub href:          String,
    pub id:            String,
//  pub type:            String, TODO Use serde instead of rustc_serialize
    pub uri:           String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Followers {
    pub href:  Option<String>,
    pub total: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaylistTrack {
    pub added_at: Option<String>,
    pub added_by: Option<User>,
    pub is_local: bool,
    pub track:    Track,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PagingObject<T> {
    pub href:     String,
    pub items:    Vec<T>,
    pub limit:    i32,
    pub next:     Option<String>,
    pub offset:   i32,
    pub previous: Option<String>,
    pub total:    i32,
}

impl<T: DeserializeOwned> PagingObject<T> {
    pub fn fetch_next(&self) -> serde_json::Result<PagingObject<T>> {
        if let Some(ref path) = self.next {
            fetch(&path).and_then(|s| serde_json::from_str(&s))
        } else {
            Err(serde_json::Error::missing_field("no value"))
        }
    }
    pub fn fetch_previous(&self) -> serde_json::Result<PagingObject<T>> {
        if let Some(ref path) = self.next {
            fetch(&path).and_then(|s| serde_json::from_str(&s))
        } else {
            Err(serde_json::Error::missing_field("no value"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    pub access_token: String,
    pub token_type:   String,
    pub expires_in:   i64,
    pub expires_at:   Option<NaiveDateTime>,
}

pub fn parse_uri_as_playlist(uri: &str) -> Option<(String, String)> {
    Regex::new(PLAYLIST_URI).ok().and_then(|re| re.captures(uri).map(|cap| {
        let user_id     = cap[2].to_string();
        let playlist_id = cap[3].to_string();
        (user_id, playlist_id)
    }))
}

pub fn parse_open_url_as_playlist(url: &str) -> Option<(String, String)> {
    Regex::new(PLAYLIST_OPEN).ok().and_then(|re| re.captures(url).map(|cap| {
        let user_id     = cap[2].to_string();
        let playlist_id = cap[3].to_string();
        (user_id, playlist_id)
    }))
}

pub fn parse_embed_url_as_playlist(url: &str) -> Option<(String, String)> {
    Regex::new(PLAYLIST_EMBED).ok().and_then(|re| re.captures(url).map(|cap| {
        let user_id     = cap[2].to_string();
        let playlist_id = cap[3].to_string();
        (user_id, playlist_id)
    }))
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
pub fn fetch_track(id: &str) -> serde_json::Result<Track> {
    let path = format!("/tracks/{}", id);
    fetch(&path).and_then(|s| serde_json::from_str(&s))
}

pub fn fetch_tracks(ids: Vec<String>) -> serde_json::Result<Vec<Track>> {
    let path = format!("/tracks?ids={}", ids.join(","));
    let result: serde_json::Result<Tracks> = fetch(&path).and_then(|s| {
        serde_json::from_str(&s)
    });
    result.map(|tracks| tracks.tracks)
}

pub fn fetch_playlist_tracks(user_id: &str, id: &str) -> serde_json::Result<PagingObject<Track>> {
    let path = format!("/users/{}/playlists/{}/tracks", user_id, id);
    fetch(&path).and_then(|s| serde_json::from_str(&s))
}

/// This function fetches a album info with spotify api.
///
/// # Examples
///
/// ```
/// let album = pink_spider::spotify::fetch_album("7exbVQgdNqseHtGCf6mZk5").unwrap();
///
/// assert_eq!(album.id, "7exbVQgdNqseHtGCf6mZk5");
/// ```
pub fn fetch_album(id: &str) -> serde_json::Result<Album> {
    let path = format!("/albums/{}", id);
    fetch(&path).and_then(|s| serde_json::from_str(&s))
}

/// This function fetches a playlist info with spotify api.
///
/// # Examples
///
/// ```
/// let playlist = pink_spider::spotify::fetch_playlist("spincoaster", "182jSXyIDGLOYwE7PLhxjI").unwrap();
///
/// assert_eq!(playlist.id, "182jSXyIDGLOYwE7PLhxjI");
///
/// assert_eq!(playlist.tracks.total, 101);
/// ```
pub fn fetch_playlist(user_id: &str, id: &str) -> serde_json::Result<Playlist> {
    let path = format!("/users/{}/playlists/{}", user_id, id);
    fetch(&path).and_then(|s| serde_json::from_str(&s))
}

pub fn fetch_artist(id: &str) -> serde_json::Result<Artist> {
    let path = format!("/artists/{}", id);
    fetch(&path).and_then(|s| serde_json::from_str(&s))
}

pub fn fetch_artists(ids: Vec<String>) -> serde_json::Result<Vec<Artist>> {
    let path = format!("/artists?ids={}", ids.join(","));
    let result: serde_json::Result<Artists> = fetch(&path).and_then(|s| {
        serde_json::from_str(&s)
    });
    result.map(|artists| artists.artists)
}

fn fetch(path: &str) -> serde_json::Result<String> {
    let url = format!("{}{}", BASE_URL, path);
    fetch_url(&url)
}

fn fetch_url(url: &str) -> serde_json::Result<String> {
    let token       = update_token_if_needed()?;
    let mut headers = Headers::new();
    headers.set(
        Authorization(
            Bearer {
                token: token.access_token
            }
        )
    );
    headers.set(Connection::close());
    let mut res = http::client().get(url)
                                .headers(headers)
                                .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    Ok(body)
}

/// This function fetches a oauth token info with spotify api.
///
/// # Examples
///
/// ```
/// let token = pink_spider::spotify::fetch_token();
/// assert!(token.is_ok());
/// ```
pub fn fetch_token() -> serde_json::Result<Token> {
    let url         = "https://accounts.spotify.com/api/token";
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
    let mut res = http::client().post(url)
                                .body("grant_type=client_credentials")
                                .headers(headers)
                                .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    serde_json::from_str::<Token>(&body)
}

pub fn get_valid_token() -> Option<Token> {
    TOKEN.lock().unwrap().clone().and_then(
        |t|
        if let Some(expires_at) = t.expires_at {
            if expires_at > Utc::now().naive_utc() {
                Some(t)
            } else {
                None
            }
        } else {
            None
        }
    )
}

pub fn update_token_if_needed() -> serde_json::Result<Token> {
    match get_valid_token() {
        Some(token) => Ok(token),
        None => match fetch_token() {
            Ok(token) => {
                let mut t        = TOKEN.lock().unwrap();
                let mut token    = token.clone();
                let now          = Utc::now().naive_utc();
                let exires_at    = now + Duration::seconds(token.expires_in);
                token.expires_at = Some(exires_at);
                *t = Some(token.clone());
                Ok(token)
            },
            Err(_) =>
                Err(serde_json::error::Error::custom("missing token".to_string()))
        },
    }
}
