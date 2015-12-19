extern crate rustc_serialize;
extern crate postgres;

use self::postgres::{Connection, SslMode};
use std::collections::BTreeMap;
use std::env;
use self::rustc_serialize::json::{ToJson, Json};

static DEFAULT_DATABASE_URL: &'static str = "postgres://pink_spider:pinkspider@localhost";

use youtube;
use soundcloud;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Track {
    pub id:         i32,
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
        d.insert("id".to_string(),         self.id.to_json());
        d.insert("provider".to_string(),   self.provider.to_string().to_json());
        d.insert("identifier".to_string(), self.identifier.to_json());
        d.insert("title".to_string(),      self.title.to_json());
        d.insert("url".to_string(),        self.url.to_json());
        Json::Object(d)
    }
}

impl Track {
    pub fn from_yt_playlist_item(item: &youtube::PlaylistItem) -> Track {
        let identifier = (*item).snippet.resourceId["videoId"].to_string();
        Track {
                    id: 0,
              provider: Provider::YouTube,
                 title: (*item).snippet.title.to_string(),
                   url: format!("https://www.youtube.com/watch/?v={}", identifier),
            identifier: identifier
        }
    }
    pub fn from_sc_track(track: &soundcloud::Track) -> Track {
        Track {
                    id: (*track).id,
              provider: Provider::SoundCloud,
                 title: (*track).title.to_string(),
                   url: (*track).permalink_url.to_string(),
            identifier: (*track).id.to_string()
        }
    }
    pub fn find_by_id(id: i32) -> Option<Track> {
        let conn = conn();
        let stmt = conn.prepare("SELECT id, provider, title, url, identifier
                                 FROM track WHERE id = $1").unwrap();
        for row in stmt.query(&[&id]).unwrap() {
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
        let conn = conn();
        let stmt = conn.prepare("SELECT id,  provider, title, url, identifier
                                 FROM track WHERE provider = $1
                                 AND identifier = $2").unwrap();
        for row in stmt.query(&[&(*provider).to_string(), &identifier]).unwrap() {
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

    pub fn find_by_entry_id(entry_id: i32) -> Vec<Track> {
        let mut tracks = Vec::new();
        let conn = conn();
        println!(" entry_id {}", entry_id);
        let stmt = conn.prepare("SELECT t.id,
                                        t.provider,
                                        t.title,
                                        t.url,
                                        t.identifier
                                 FROM track t LEFT JOIN track_entry te
                                 ON t.id = te.track_id
                                 WHERE te.entry_id = $1").unwrap();
        for row in stmt.query(&[&entry_id]).unwrap() {
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
        let conn = conn();
        let stmt = conn.prepare("SELECT id, provider, title, url, identifier FROM track").unwrap();
        for row in stmt.query(&[]).unwrap() {
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
        let conn = conn();
        let stmt = conn.prepare("INSERT INTO track (provider, title, url, identifier)
                                 VALUES ($1, $2, $3, $4) RETURNING id").unwrap();
        for row in stmt.query(&[&provider.to_string(), &title, &url, &identifier]).unwrap() {
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
        let conn = conn();
        let stmt = conn.prepare("UPDATE track SET title=$1, url=$2
                                 WHERE id = $3").unwrap();
        match stmt.query(&[&self.title, &self.url, &self.id]) {
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
    pub id:  i32,
    pub url: String,
    pub tracks: Vec<Track>,
}

impl ToJson for Entry {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("id".to_string(),  self.id.to_json());
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
        let conn = conn();
        let stmt = conn.prepare("SELECT id, url FROM entry WHERE id = $1").unwrap();
        for row in stmt.query(&[&id]).unwrap() {
            return Some(Entry {
                    id: row.get(0),
                   url: row.get(1),
                tracks: Track::find_by_entry_id(row.get(0))
            });
        }
        return None
    }

    pub fn find_by_url(url: &str) -> Option<Entry> {
        let conn = conn();
        let stmt = conn.prepare("SELECT id, url FROM entry WHERE url = $1").unwrap();
        for row in stmt.query(&[&url]).unwrap() {
            let id = row.get(0);
            return Some(Entry {
                    id: id,
                   url: row.get(1),
                tracks: Track::find_by_entry_id(id)
            });
        }
        return None
    }

    pub fn find_all() -> Vec<Entry> {
        let conn = conn();
        let stmt = conn.prepare("SELECT id, url FROM entry").unwrap();
        let mut entries = Vec::new();
        for row in stmt.query(&[]).unwrap() {
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
        let conn = conn();
        let stmt = conn.prepare("INSERT INTO entry (url) VALUES ($1) RETURNING id").unwrap();
        for row in stmt.query(&[&url]).unwrap() {
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
        let conn = conn();
        let stmt = conn.prepare("INSERT INTO track_entry (track_id, entry_id)
                                 VALUES ($1, $2)").unwrap();
        stmt.query(&[&track.id, &self.id]).unwrap();
        self.tracks.push(track);
    }

    pub fn save(&self) -> bool {
        return true
    }
}

pub fn conn() -> Connection {
    let opt_url = env::var("DATABASE_URL");
    match opt_url {
        Ok(url)  =>
            Connection::connect(url.trim(), &SslMode::None).unwrap(),
        Err(_) =>
            Connection::connect(DEFAULT_DATABASE_URL, &SslMode::None).unwrap()
    }
}

pub fn create_tables() {
    let conn = conn();

    match conn.execute("CREATE TABLE IF NOT EXISTS
                        track (id         SERIAL PRIMARY KEY,
                               provider   VARCHAR NOT NULL,
                               identifier VARCHAR NOT NULL,
                               title      VARCHAR NOT NULL,
                               url        VARCHAR NOT NULL)", &[]) {
        Ok(_) => println!("Succeeded in creating track table if not exists"),
        Err(error) => println!("error {}", error)
    }

    match conn.execute("CREATE TABLE IF NOT EXISTS
                        entry (id  SERIAL PRIMARY KEY,
                        url VARCHAR NOT NULL)", &[]) {
        Ok(_) => println!("Succeeded in creating entry table if not exists"),
        Err(error) => println!("error {}", error)
    }

    match conn.execute("CREATE TABLE IF NOT EXISTS
                        track_entry (id  SERIAL PRIMARY KEY,
                        track_id SERIAL NOT NULL,
                        entry_id SERIAL NOT NULL)", &[]) {
        Ok(_) => println!("Succeeded in creating track_entry table if not exists "),
        Err(error) => println!("error {}", error)
    }
}

pub fn drop_tables() {
    let conn = conn();
    match conn.execute("DROP TABLE track", &[]) {
        Ok(_) => println!("Succeeded in dropping track table"),
        Err(error) => println!("Failed to drop error {}", error)
    }
    match conn.execute("DROP TABLE entry", &[]) {
        Ok(_) => println!("Succeeded in dropping entry table"),
        Err(error) => println!("error {}", error)
    }
    match conn.execute("DROP TABLE track_entry", &[]) {
        Ok(_) => println!("Succeeded in dropping track_entry table"),
        Err(error) => println!("error {}", error)
    }
}
