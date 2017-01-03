use std::collections::BTreeMap;
use rustc_serialize::json::{ToJson, Json};
use postgres;
use uuid::Uuid;
use std::fmt;

use youtube;
use soundcloud;
use error::Error;
use super::{conn, PaginatedCollection};

#[derive(Debug, Copy, Clone)]
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
    pub id:            Uuid,
    pub provider:      Provider,
    pub identifier:    String,
    pub url:           String,
    pub title:         String,
    pub description:   Option<String>,
    pub artist:        Option<String>,
    pub thumbnail_url: Option<String>,
    pub artwork_url:   Option<String>,
    pub duration:      i32,
}

impl PartialEq for Track {
    fn eq(&self, t: &Track) -> bool {
        return self.identifier == t.identifier && self.provider == t.provider
    }
}

impl ToJson for Track {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("id".to_string()           , self.id.to_string().to_json());
        d.insert("provider".to_string()     , self.provider.to_json());
        d.insert("identifier".to_string()   , self.identifier.to_json());
        d.insert("url".to_string()          , self.url.to_json());
        d.insert("title".to_string()        , self.title.to_json());
        d.insert("description".to_string()  , self.description.to_json());
        d.insert("artist".to_string()       , self.artist.to_json());
        d.insert("thumbnail_url".to_string(), self.thumbnail_url.to_json());
        d.insert("artwork_url".to_string()  , self.artwork_url.to_json());
        d.insert("duration".to_string()     , self.duration.to_json());
        Json::Object(d)
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.provider, self.identifier)
    }
}

impl Track {
    pub fn new(provider: Provider, identifier: String) -> Track {
        Track {
            id:            Uuid::new_v4(),
            provider:      provider,
            identifier:    identifier,
            url:           "".to_string(),
            title:         "".to_string(),
            description:   None,
            artist:        None,
            thumbnail_url: None,
            artwork_url:   None,
            duration:      0,
        }
    }
    pub fn from_yt_playlist_item(item: &youtube::PlaylistItem) -> Track {
        let identifier = (*item).snippet.resourceId["videoId"].to_string();
        Track::new(Provider::YouTube, identifier.to_string())
            .update_with_yt_playlist_item(item)
            .clone()
    }
    pub fn from_sc_track(track: &soundcloud::Track) -> Track {
        Track::new(Provider::SoundCloud, (*track).id.to_string())
            .update_with_sc_track(track)
            .clone()
    }

    pub fn update_with_yt_video(&mut self, video: &youtube::Video) -> &mut Track {
        self.provider      = Provider::YouTube;
        self.identifier    = video.id.to_string();
        self.url           = format!("https://www.youtube.com/watch/?v={}", video.id);
        self.title         = video.snippet.title.to_string();
        self.description   = Some(video.snippet.description.to_string());
        self.artist        = Some(video.snippet.channelTitle.to_string());
        self.thumbnail_url = Some(video.snippet.thumbnails["default"].url.to_string());
        self.artwork_url   = Some(video.snippet.thumbnails["maxres"].url.to_string());
        self
    }

    pub fn update_with_yt_playlist_item(&mut self, item: &youtube::PlaylistItem) -> &mut Track {
        self.provider      = Provider::YouTube;
        self.identifier    = item.id.to_string();
        self.url           = format!("https://www.youtube.com/watch/?v={}", item.id);
        self.title         = item.snippet.title.to_string();
        self.description   = Some(item.snippet.description.to_string());
        self.artist        = Some(item.snippet.channelTitle.to_string());
        self.thumbnail_url = Some(item.snippet.thumbnails["default"].url.to_string());
        self.artwork_url   = Some(item.snippet.thumbnails["maxres"].url.to_string());
        self
    }


    pub fn update_with_sc_track(&mut self, track: &soundcloud::Track) -> &mut Track {
        self.provider      = Provider::SoundCloud;
        self.identifier    = track.id.to_string();
        self.url           = track.permalink_url.to_string();
        self.title         = track.title.to_string();
        self.description   = Some(track.description.to_string());
        self.artist        = track.artist.clone();
        self.thumbnail_url = track.artwork_url.clone();
        self.artwork_url   = track.artwork_url.clone();
        self
    }

    fn rows_to_tracks(rows: postgres::rows::Rows) -> Vec<Track> {
        let mut tracks = Vec::new();
        for row in rows.iter() {
            let track = Track {
                id:            row.get(0),
                provider:      Provider::new(row.get(1)),
                identifier:    row.get(2),
                url:           row.get(3),
                title:         row.get(4),
                description:   row.get(5),
                artist:        row.get(6),
                thumbnail_url: row.get(7),
                artwork_url:   row.get(8),
                duration:      row.get(9)
            };
            tracks.push(track)
        }
        tracks
    }

    pub fn find_by_id(id: &str) -> Result<Track, Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("SELECT id, provider, identifier, url, title, description,
                                        artist, thumbnail_url, artwork_url, duration
                                 FROM tracks WHERE id = $1").unwrap();
        let uuid   = try!(Uuid::parse_str(id).map_err(|_| Error::Unprocessable));
        let rows   = stmt.query(&[&uuid]).unwrap();
        let tracks = Track::rows_to_tracks(rows);
        if tracks.len() > 0 {
            return Ok(tracks[0].clone());
        }
        return Err(Error::NotFound)
    }

    pub fn find_by(provider: &Provider, identifier: &str) -> Result<Track, Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("SELECT id, provider, identifier, url, title, description,
                                        artist, thumbnail_url, artwork_url, duration
                                 FROM tracks WHERE provider = $1
                                 AND identifier = $2").unwrap();
        let rows = stmt.query(&[&(*provider).to_string(), &identifier]).unwrap();
        let tracks = Track::rows_to_tracks(rows);
        if tracks.len() > 0 {
            return Ok(tracks[0].clone());
        }
        return Err(Error::NotFound)
    }

    pub fn find_by_entry_id(entry_id: Uuid) -> Vec<Track> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("SELECT t.id, t.provider, t.identifier, t.url, t.title, t.description,
                                        t.artist, t.thumbnail_url, t.artwork_url, t.duration
                                 FROM tracks t LEFT JOIN track_entries te
                                 ON t.id = te.track_id
                                 WHERE te.entry_id = $1").unwrap();
        let rows = stmt.query(&[&entry_id]).unwrap();
        Track::rows_to_tracks(rows)
    }

    pub fn find(page: i64, per_page: i64) -> PaginatedCollection<Track> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("SELECT id, provider, identifier, url, title, description,
                                        artist, thumbnail_url, artwork_url, duration
                                 FROM tracks LIMIT $2 OFFSET $1").unwrap();
        let offset = page * per_page;
        let rows   = stmt.query(&[&offset, &per_page]).unwrap();
        let tracks = Track::rows_to_tracks(rows);
        let mut total: i64 = 0;
        for row in conn.query("SELECT COUNT(*) FROM tracks", &[]).unwrap().iter() {
            total = row.get(0);
        }
        PaginatedCollection {
            page:     page,
            per_page: per_page,
            total:    total,
            items:    tracks,
        }
    }

    pub fn create(&self) -> Result<Track, Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("INSERT INTO tracks (provider, identifier, url, title)
                                 VALUES ($1, $2, $3, $4) RETURNING id").unwrap();
        let rows = try!(stmt.query(&[&self.provider.to_string(), &self.identifier, &self.url, &self.title]));
        let mut track = self.clone();
        for row in rows.iter() {
            track.id = row.get(0);
        }
        Ok(track)
    }

    pub fn find_or_create(provider: Provider, identifier: String) -> Result<Track, Error> {
        return match Track::find_by(&provider, &identifier) {
            Ok(track) => Ok(track),
            Err(_)    => Track::new(provider, identifier).create()
        }
    }

    pub fn save(&self) -> Result<(), Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("UPDATE tracks SET
                                 provider      = $2,
                                 identifier    = $3,
                                 url           = $4,
                                 title         = $5,
                                 description   = $6,
                                 artist        = $7,
                                 thumbnail_url = $8,
                                 artwork_url   = $9,
                                 duration      = $10
                                 WHERE id = $1").unwrap();
        let result = stmt.query(&[&self.id,
                                  &self.provider.to_string(),
                                  &self.identifier,
                                  &self.url,
                                  &self.title,
                                  &self.description,
                                  &self.artist,
                                  &self.thumbnail_url,
                                  &self.artwork_url,
                                  &self.duration]);
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

