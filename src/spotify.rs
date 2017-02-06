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
    pub height: i32,
    pub url:    String,
    pub width:  i32,
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct TrackLink {
    pub external_urls: BTreeMap<String, String>,
    pub href:          String,
    pub id:            String,
//  pub type:            String, TODO Use serde instead of rustc_serialize
    pub uri:           String,
}
