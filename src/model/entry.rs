use std::collections::BTreeMap;
use rustc_serialize::json::{ToJson, Json};
use uuid::Uuid;
use error::Error;
use super::conn;
use Track;
use super::open_graph;

#[derive(Debug)]
pub struct Entry {
    pub id:      Uuid,
    pub url:     String,
    pub tracks:  Vec<Track>,

    pub og_obj: Option<open_graph::Object>,
}

impl ToJson for Entry {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("id".to_string(),  self.id.to_string().to_json());
        d.insert("url".to_string(), self.url.to_json());
        d.insert("tracks".to_string(),
                 Json::Array(self.tracks.iter().map(|x| x.to_json()).collect()));

        d.insert("og_object".to_string()  , self.og_obj.to_json());
        Json::Object(d)
    }
}

impl Entry {
    pub fn find_by_id(id: String) -> Result<Entry, Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("SELECT id, url FROM entries WHERE id = $1").unwrap();
        for row in stmt.query(&[&id]).unwrap().iter() {
            return Ok(Entry {
                    id: row.get(0),
                   url: row.get(1),
                tracks: Track::find_by_entry_id(row.get(0)),
                og_obj: None,
            });
        }
        return Err(Error::NotFound)
    }

    pub fn find_by_url(url: &str) -> Result<Entry, Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("SELECT id, url FROM entries WHERE url = $1").unwrap();
        for row in stmt.query(&[&url]).unwrap().iter() {
            return Ok(Entry {
                    id: row.get(0),
                   url: row.get(1),
                tracks: Track::find_by_entry_id(row.get(0)),
                og_obj: None,
            });
        }
        return Err(Error::NotFound)
    }

    pub fn find_all() -> Vec<Entry> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("SELECT id, url FROM entries").unwrap();
        let mut entries = Vec::new();
        for row in stmt.query(&[]).unwrap().iter() {
            entries.push(Entry {
                    id: row.get(0),
                   url: row.get(1),
                tracks: Track::find_by_entry_id(row.get(0)),
                og_obj: None,
            })
        }
        return entries
    }

    pub fn find_or_create_by_url(url: String) -> Result<Entry, Error> {
        match Entry::find_by_url(&url) {
            Ok(entry) => Ok(entry),
            Err(_)    => Entry::create_by_url(url)
        }
    }

    pub fn create_by_url(url: String) -> Result<Entry, Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("INSERT INTO entries (url) VALUES ($1) RETURNING id").unwrap();
        for row in stmt.query(&[&url]).unwrap().iter() {
            let entry = Entry {
                      id: row.get(0),
                     url: url,
                  tracks: Vec::new(),
                  og_obj: None,
            };
            return Ok(entry);
        }
        Err(Error::Unexpected)
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
