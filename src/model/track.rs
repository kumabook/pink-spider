use std::collections::BTreeMap;
use rustc_serialize::json::{ToJson, Json};
use uuid::Uuid;
use std::fmt;

use youtube;
use soundcloud;
use error::Error;

use super::conn;

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
            Provider::YouTube    => "YouTube",
            Provider::SoundCloud => "SoundCloud",
            Provider::Raw        => "Raw",
        }.to_string()
    }
    pub fn new(str: String) -> Provider {
        match str.as_ref() {
            "YouTube"    => Provider::YouTube,
            "youtube"    => Provider::YouTube,
            "SoundCloud" => Provider::SoundCloud,
            "soundcloud" => Provider::SoundCloud,
            _            => Provider::Raw,
        }
    }
}

impl ToJson for Provider {
    fn to_json(&self) -> Json {
        self.to_string().to_json()
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
        d.insert("provider".to_string(),   self.provider.to_json());
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
    pub fn find_by_id(id: &str) -> Result<Track, Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("SELECT id, provider, title, url, identifier
                                 FROM tracks WHERE id = $1").unwrap();
        let uuid = try!(Uuid::parse_str(id).map_err(|_| Error::Unprocessable));
        for row in stmt.query(&[&uuid]).unwrap().iter() {
            let track = Track {
                      id: row.get(0),
                provider: Provider::new(row.get(1)),
                   title: row.get(2),
                     url: row.get(3),
              identifier: row.get(4)
            };
            return Ok(track);
        }
        return Err(Error::NotFound)
    }

    pub fn find_by(provider: &Provider, identifier: &str) -> Result<Track, Error> {
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
            return Ok(track);
        }
        return Err(Error::NotFound)
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

    pub fn create(provider: Provider,
                  title: String,
                  url: String,
                  identifier: String) -> Result<Track, Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("INSERT INTO tracks (provider, title, url, identifier)
                                 VALUES ($1, $2, $3, $4) RETURNING id").unwrap();
        let rows = try!(stmt.query(&[&provider.to_string(), &title, &url, &identifier]));
        for row in rows.iter() {
            let track = Track {
                        id: row.get(0),
                  provider: provider,
                     title: title,
                       url: url,
                identifier: identifier
            };
            return Ok(track);
        }
        Err(Error::Unexpected)
    }

    pub fn find_or_create(provider: Provider,
                          title: String,
                          url: String,
                          identifier: String) -> Result<Track, Error> {
        return match Track::find_by(&provider, &identifier) {
            Ok(track) => Ok(track),
            Err(_)    => Track::create(provider, title, url, identifier)
        }
    }

    pub fn save(&self) -> Result<(), Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("UPDATE track SET title=$1, url=$2 WHERE id = $3").unwrap();
        let result = stmt.query(&[&self.title, &self.url, &self.id.to_string()]);
        match result {
            Ok(_)  => Ok(()),
            Err(_) => Err(Error::Unexpected)
        }
    }
}

pub struct Playlist {
    pub title:  String,
    pub tracks: Vec<Track>,
}

