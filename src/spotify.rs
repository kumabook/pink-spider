use std::io::Read;
use std::collections::BTreeMap;
use rustc_serialize::json;
use hyper::Client;
use hyper::header::Connection;

static BASE_URL: &'static str = "https://api.spotify.com/v1";

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
    pub preview_url:       String,
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
    let url    = format!("{}/tracks/{}", BASE_URL, id);
    let client = Client::new();
    let mut res = client.get(&url)
                        .header(Connection::close())
                        .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    json::decode::<Track>(&body)
}
