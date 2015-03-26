extern crate serialize;
extern crate postgres;

use self::postgres::{Connection, SslMode};
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

pub struct Entry {
    pub url: String,
    pub tracks: Vec<Track>,
}

impl ToJson for Entry {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("url".to_string(), self.url.to_json());
        let ref tracks = self.tracks;
        let mut t = Vec::new();
        for ref x in tracks.iter() {
            t.push(x.to_json());
        }
        d.insert("tracks".to_string(), Json::Array(t));
        Json::Object(d)
    }
}


pub fn create_tables() {
    let conn = Connection::connect("postgres://postgres@localhost",
                                   &SslMode::None)
        .unwrap();

    conn.execute("CREATE TABLE track (id         SERIAL PRIMARY KEY,
                                      provider   VARCHAR NOT NULL,
                                      service_id VARCHAR NOT NULL,
                                      title      VARCHAR NOT NULL,
                                      url        VARCHAR NOT NULL)", &[]).unwrap();

    conn.execute("CREATE TABLE entry (id  SERIAL PRIMARY KEY,
                                      url VARCHAR NOT NULL)", &[]).unwrap();

    conn.execute("CREATE TABLE track_entry (id  SERIAL PRIMARY KEY,
                                            track_id SERIAL NOT NULL,
                                            entry_id SERIAL NOT NULL)", &[]).unwrap();
}
