use postgres::{Connection, SslMode};
use postgres::error::ConnectError;
use std::collections::BTreeMap;
use std::env;
use rustc_serialize::json::{ToJson, Json};
use uuid::Uuid;
use std::fmt;

static DEFAULT_DATABASE_URL: &'static str = "postgres://kumabook@localhost/pink_spider_development_master";

use youtube;
use soundcloud;

#[derive(Debug, Clone)]
pub enum Provider {
    YouTube,
    SoundCloud,
    Raw
}
impl PartialEq for Provider {
    fn eq(&self, p: &Provider) -> bool {
        match *self {
            Provider::YouTube    => match *p { Provider::YouTube    => true, _ => false },
            Provider::SoundCloud => match *p { Provider::SoundCloud => true, _ => false },
            Provider::Raw        => match *p { Provider::Raw        => true, _ => false }
        }
    }
}
impl Provider {
    fn to_string(&self) -> String {
        match *self {
            Provider::YouTube    => "YouTube".to_string(),
            Provider::SoundCloud => "SoundCloud".to_string(),
            Provider::Raw        => "Raw".to_string(),
        }
    }
    fn new(str: String) -> Provider {
        match str.as_ref() {
            "YouTube"    => Provider::YouTube,
            "SoundCloud" => Provider::SoundCloud,
            _            => Provider::Raw,
        }
    }
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Track {
    pub id:         Uuid,
    pub provider:   Provider,
    pub title:      String,
    pub url:        String,
    pub identifier: String
}

impl PartialEq for Track {
    fn eq(&self, t: &Track) -> bool {
        return self.identifier == t.identifier && self.provider == t.provider
    }
}

impl ToJson for Track {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("id".to_string(),         self.id.to_string().to_json());
        d.insert("provider".to_string(),   self.provider.to_string().to_json());
        d.insert("identifier".to_string(), self.identifier.to_json());
        d.insert("title".to_string(),      self.title.to_json());
        d.insert("url".to_string(),        self.url.to_json());
        Json::Object(d)
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.provider, self.identifier)
    }
}

impl Track {
    pub fn from_yt_playlist_item(item: &youtube::PlaylistItem) -> Track {
        let identifier = (*item).snippet.resourceId["videoId"].to_string();
        Track {
                    id: Uuid::new_v4(),
              provider: Provider::YouTube,
                 title: (*item).snippet.title.to_string(),
                   url: format!("https://www.youtube.com/watch/?v={}", identifier),
            identifier: identifier
        }
    }
    pub fn from_sc_track(track: &soundcloud::Track) -> Track {
        Track {
                    id: Uuid::new_v4(),
              provider: Provider::SoundCloud,
                 title: (*track).title.to_string(),
                   url: (*track).permalink_url.to_string(),
            identifier: (*track).id.to_string()
        }
    }
    pub fn find_by_id(id: i32) -> Option<Track> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("SELECT id, provider, title, url, identifier
                                 FROM tracks WHERE id = $1").unwrap();
        for row in stmt.query(&[&id]).unwrap().iter() {
            let track = Track {
                      id: row.get(0),
                provider: Provider::new(row.get(1)),
                   title: row.get(2),
                     url: row.get(3),
              identifier: row.get(4)
            };
            return Some(track);
        }
        return None
    }

    pub fn find_by(provider: &Provider, identifier: &str) -> Option<Track> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("SELECT id,  provider, title, url, identifier
                                 FROM tracks WHERE provider = $1
                                 AND identifier = $2").unwrap();
        for row in stmt.query(&[&(*provider).to_string(), &identifier]).unwrap().iter() {
            let track = Track {
                      id: row.get(0),
                provider: Provider::new(row.get(1)),
                   title: row.get(2),
                     url: row.get(3),
              identifier: row.get(4)
            };
            return Some(track);
        }
        return None
    }

    pub fn find_by_entry_id(entry_id: Uuid) -> Vec<Track> {
        let mut tracks = Vec::new();
        let conn = conn().unwrap();
        let stmt = conn.prepare("SELECT t.id,
                                        t.provider,
                                        t.title,
                                        t.url,
                                        t.identifier
                                 FROM tracks t LEFT JOIN track_entries te
                                 ON t.id = te.track_id
                                 WHERE te.entry_id = $1").unwrap();
        for row in stmt.query(&[&entry_id]).unwrap().iter() {
            tracks.push(Track {
                      id: row.get(0),
                provider: Provider::new(row.get(1)),
                   title: row.get(2),
                     url: row.get(3),
              identifier: row.get(4)
            });
        }
        return tracks
    }

    pub fn find_all() -> Vec<Track> {
        let mut tracks = Vec::new();
        let conn = conn().unwrap();
        let stmt = conn.prepare("SELECT id, provider, title, url, identifier FROM track").unwrap();
        for row in stmt.query(&[]).unwrap().iter() {
            let track = Track {
                      id: row.get(0),
                provider: Provider::new(row.get(1)),
                   title: row.get(2),
                     url: row.get(3),
              identifier: row.get(4)
            };
            tracks.push(track);
        }
        return tracks
    }

    pub fn create(provider: Provider, title: String, url: String, identifier: String) -> Option<Track> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("INSERT INTO tracks (provider, title, url, identifier)
                                 VALUES ($1, $2, $3, $4) RETURNING id").unwrap();
        for row in stmt.query(&[&provider.to_string(), &title, &url, &identifier]).unwrap().iter() {
            let track = Track {
                        id: row.get(0),
                  provider: provider,
                     title: title,
                       url: url,
                identifier: identifier
            };
            return Some(track);
        }
        return None
    }

    pub fn find_or_create(provider: Provider, title: String, url: String, identifier: String) -> Option<Track> {
        return match Track::find_by(&provider, &identifier) {
            Some(track) => Some(track),
            None        => Track::create(provider, title, url, identifier)
        }
    }

    pub fn save(&self) -> bool {
        let conn = conn().unwrap();
        let stmt = conn.prepare("UPDATE track SET title=$1, url=$2 WHERE id = $3").unwrap();
        let result = stmt.query(&[&self.title, &self.url, &self.id.to_string()]);
        match result {
            Ok(_)  => return true,
            Err(_) => return false
        }
    }
}

pub struct Playlist {
    pub title:  String,
    pub tracks: Vec<Track>,
}

#[derive(Debug)]
pub struct Entry {
    pub id:  Uuid,
    pub url: String,
    pub tracks: Vec<Track>,
}

impl ToJson for Entry {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("id".to_string(),  self.id.to_string().to_json());
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

impl Entry {
    pub fn find_by_id(id: String) -> Option<Entry> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("SELECT id, url FROM entries WHERE id = $1").unwrap();
        for row in stmt.query(&[&id]).unwrap().iter() {
            return Some(Entry {
                    id: row.get(0),
                   url: row.get(1),
                tracks: Track::find_by_entry_id(row.get(0))
            });
        }
        return None
    }

    pub fn find_by_url(url: &str) -> Option<Entry> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("SELECT id, url FROM entries WHERE url = $1").unwrap();
        for row in stmt.query(&[&url]).unwrap().iter() {
            return Some(Entry {
                    id: row.get(0),
                   url: row.get(1),
                tracks: Track::find_by_entry_id(row.get(0))
            });
        }
        return None
    }

    pub fn find_all() -> Vec<Entry> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("SELECT id, url FROM entries").unwrap();
        let mut entries = Vec::new();
        for row in stmt.query(&[]).unwrap().iter() {
            entries.push(Entry {
                    id: row.get(0),
                   url: row.get(1),
                tracks: Track::find_by_entry_id(row.get(0))
            })
        }
        return entries
    }

    pub fn find_or_create_by_url(url: String) -> Option<Entry> {
        return match Entry::find_by_url(&url) {
            Some(entry) => Some(entry),
            None        => Entry::create_by_url(url)
        }
    }

    pub fn create_by_url(url: String) -> Option<Entry> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("INSERT INTO entries (url) VALUES ($1) RETURNING id").unwrap();
        for row in stmt.query(&[&url]).unwrap().iter() {
            let entry = Entry {
                      id: row.get(0),
                     url: url,
                  tracks: Vec::new()
            };
            return Some(entry);
        }
        return None
    }

    pub fn add_track(&mut self, track: Track) {
        let conn = conn().unwrap();
        let stmt = conn.prepare("INSERT INTO track_entries (track_id, entry_id)
                                 VALUES ($1, $2)").unwrap();
        stmt.query(&[&track.id, &self.id]).unwrap();
        self.tracks.push(track);
    }

    pub fn save(&self) -> bool {
        return true
    }
}

pub fn conn() -> Result<Connection, ConnectError> {
    let opt_url = env::var("DATABASE_URL");
    match opt_url {
        Ok(url) =>
            Connection::connect(url.trim(), SslMode::None),
        Err(_)  =>
            Connection::connect(DEFAULT_DATABASE_URL, SslMode::None)
    }
}
