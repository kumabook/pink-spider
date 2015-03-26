extern crate serialize;

use std::collections::BTreeMap;
use self::serialize::json::{ToJson, Json};

pub enum Provider {
    YouTube,
    SoundCloud,
    Raw
}
impl Provider {
    fn to_string(&self) -> String {
        match *self {
            Provider::YouTube    => "YouTube".to_string(),
            Provider::SoundCloud => "SoundCloud".to_string(),
            Provider::Raw        => "Raw".to_string(),
        }
    }
}

pub struct Track {
    pub provider:  Provider,
    pub title:     String,
    pub url:       String,
    pub service_id: String
}
impl ToJson for Track {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("provider".to_string(),   self.provider.to_string().to_json());
        d.insert("service_id".to_string(), self.service_id.to_json());
        d.insert("title".to_string(),      self.title.to_json());
        d.insert("url".to_string(),        self.url.to_json());
        Json::Object(d)
    }
}

pub struct Playlist {
    pub title:  String,
    pub tracks: Vec<Track>,
}

impl ToJson for Playlist {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("title".to_string(), self.title.to_json());
        let ref tracks = self.tracks;
        let mut t = Vec::new();
        for ref x in tracks.iter() {
            t.push(x.to_json());
        }
        d.insert("tracks".to_string(), Json::Array(t));
        Json::Object(d)
    }
}
