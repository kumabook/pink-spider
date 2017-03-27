use std::collections::BTreeMap;
use rustc_serialize::json::{ToJson, Json};
use postgres;
use uuid::Uuid;
use std::fmt;
use chrono::{NaiveDateTime, UTC, DateTime};

use apple_music;
use youtube;
use youtube::HasThumbnail;
use soundcloud;
use spotify;
use error::Error;
use super::{conn, Model};
use model::enclosure::Enclosure;
use model::provider::Provider;
use model::state::State;
use model::artist::Artist;

static PROPS: [&'static str; 16]  = ["id",
                                     "provider",
                                     "identifier",
                                     "owner_id",
                                     "owner_name",
                                     "url",
                                     "title",
                                     "description",
                                     "thumbnail_url",
                                     "artwork_url",
                                     "audio_url",
                                     "duration",
                                     "published_at",
                                     "created_at",
                                     "updated_at",
                                     "state"];

#[derive(Debug, Clone)]
pub struct Track {
    pub id:            Uuid,
    pub provider:      Provider,
    pub identifier:    String,
    pub owner_id:      Option<String>,
    pub owner_name:    Option<String>,
    pub url:           String,
    pub title:         String,
    pub description:   Option<String>,
    pub thumbnail_url: Option<String>,
    pub artwork_url:   Option<String>,
    pub audio_url:     Option<String>,
    pub duration:      i32,
    pub published_at:  NaiveDateTime,
    pub created_at:    NaiveDateTime,
    pub updated_at:    NaiveDateTime,
    pub state:         State,
    pub artists:       Option<Vec<Artist>>,
}

impl PartialEq for Track {
    fn eq(&self, t: &Track) -> bool {
        return self.identifier == t.identifier && self.provider == t.provider
    }
}

impl ToJson for Track {
    fn to_json(&self) -> Json {
        let published_at = DateTime::<UTC>::from_utc(self.published_at, UTC);
        let created_at   = DateTime::<UTC>::from_utc(self.created_at  , UTC);
        let updated_at   = DateTime::<UTC>::from_utc(self.updated_at  , UTC);
        let mut d = BTreeMap::new();
        d.insert("id".to_string()           , self.id.to_string().to_json());
        d.insert("provider".to_string()     , self.provider.to_json());
        d.insert("identifier".to_string()   , self.identifier.to_json());
        d.insert("owner_id".to_string()     , self.owner_id.to_json());
        d.insert("owner_name".to_string()   , self.owner_name.to_json());
        d.insert("url".to_string()          , self.url.to_json());
        d.insert("title".to_string()        , self.title.to_json());
        d.insert("description".to_string()  , self.description.to_json());
        d.insert("thumbnail_url".to_string(), self.thumbnail_url.to_json());
        d.insert("artwork_url".to_string()  , self.artwork_url.to_json());
        d.insert("audio_url".to_string()    , self.audio_url.to_json());
        d.insert("duration".to_string()     , self.duration.to_json());
        d.insert("published_at".to_string() , published_at.to_rfc3339().to_json());
        d.insert("created_at".to_string()   , created_at.to_rfc3339().to_json());
        d.insert("updated_at".to_string()   , updated_at.to_rfc3339().to_json());
        d.insert("state".to_string()        , self.state.to_json());
        d.insert("artists".to_string()      , self.artists.to_json());
        Json::Object(d)
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.provider, self.identifier)
    }
}

impl Model for Track {
    fn table_name() -> String {
        "tracks".to_string()
    }
    fn props_str(prefix: &str) -> String {
        PROPS
            .iter()
            .map(|&p| format!("{}{}", prefix, p))
            .collect::<Vec<String>>().join(",")
    }
    fn rows_to_items(rows: postgres::rows::Rows) -> Vec<Track> {
        let mut tracks = Vec::new();
        for row in rows.iter() {
            let track = Track {
                id:            row.get(0),
                provider:      Provider::new(row.get(1)),
                identifier:    row.get(2),
                owner_id:      row.get(3),
                owner_name:    row.get(4),
                url:           row.get(5),
                title:         row.get(6),
                description:   row.get(7),
                thumbnail_url: row.get(8),
                artwork_url:   row.get(9),
                audio_url:     row.get(10),
                duration:      row.get(11),
                published_at:  row.get(12),
                created_at:    row.get(13),
                updated_at:    row.get(14),
                state:         State::new(row.get(15)),
                artists:       None,
            };
            tracks.push(track)
        }
        tracks
    }

    fn create(&self) -> Result<Track, Error> {
        let conn = try!(conn());
        let stmt = try!(conn.prepare("INSERT INTO tracks (provider, identifier, url, title)
                                      VALUES ($1, $2, $3, $4) RETURNING id"));
        let rows = try!(stmt.query(&[&self.provider.to_string(), &self.identifier, &self.url, &self.title]));
        let mut track = self.clone();
        for row in rows.iter() {
            track.id = row.get(0);
        }
        Ok(track)
    }

    fn save(&mut self) -> Result<(), Error> {
        self.updated_at = UTC::now().naive_utc();
        let conn = try!(conn());
        let stmt = try!(conn.prepare("UPDATE tracks SET
                                      provider      = $2,
                                      identifier    = $3,
                                      owner_id      = $4,
                                      owner_name    = $5,
                                      url           = $6,
                                      title         = $7,
                                      description   = $8,
                                      thumbnail_url = $9,
                                      artwork_url   = $10,
                                      audio_url     = $11,
                                      duration      = $12,
                                      published_at  = $13,
                                      created_at    = $14,
                                      updated_at    = $15,
                                      state         = $16
                                      WHERE id = $1"));
        let result = stmt.query(&[&self.id,
                                  &self.provider.to_string(),
                                  &self.identifier,
                                  &self.owner_id,
                                  &self.owner_name,
                                  &self.url,
                                  &self.title,
                                  &self.description,
                                  &self.thumbnail_url,
                                  &self.artwork_url,
                                  &self.audio_url,
                                  &self.duration,
                                  &self.published_at,
                                  &self.created_at,
                                  &self.updated_at,
                                  &self.state.to_string(),
        ]);
        match result {
            Ok(_)  => Ok(()),
            Err(_) => Err(Error::Unexpected)
        }
    }
}

impl Enclosure for Track {
    fn new(provider: Provider, identifier: String) -> Track {
        Track {
            id:            Uuid::new_v4(),
            provider:      provider,
            identifier:    identifier,
            owner_id:      None,
            owner_name:    None,
            url:           "".to_string(),
            title:         "".to_string(),
            description:   None,
            thumbnail_url: None,
            artwork_url:   None,
            audio_url:     None,
            duration:      0,
            published_at:  UTC::now().naive_utc(),
            created_at:    UTC::now().naive_utc(),
            updated_at:    UTC::now().naive_utc(),
            state:         State::Alive,
            artists:       None,
        }
    }
    fn find_by_entry_id(entry_id: Uuid) -> Vec<Track> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(
            &format!("SELECT {} FROM tracks t LEFT JOIN track_entries te
                        ON t.id = te.track_id
                        WHERE te.entry_id = $1
                        ORDER BY t.published_at DESC",
                     Track::props_str("t."))).unwrap();
        let rows = stmt.query(&[&entry_id]).unwrap();
        Track::rows_to_items(rows)
    }
}

impl Track {
    pub fn from_am_song(song: &apple_music::Song) -> Track {
        let identifier = (*song).id.to_string();
        Track::new(Provider::AppleMusic, identifier.to_string())
            .update_with_am_song(song)
            .clone()
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
    pub fn from_sp_track(track: &spotify::Track) -> Track {
        Track::new(Provider::Spotify, (*track).id.to_string())
            .update_with_sp_track(track)
            .clone()
    }
    pub fn fetch_detail(&mut self) -> &mut Track {
        match self.provider {
            Provider::YouTube => match youtube::fetch_video(&self.identifier) {
                Ok(video) => self.update_with_yt_video(&video),
                Err(_)    => self.disable(),
            },
            Provider::SoundCloud => match soundcloud::fetch_track(&self.identifier) {
                Ok(sc_track) => self.update_with_sc_track(&sc_track),
                Err(_)       => self.disable(),
            },
            _ => self,
        }
    }

    fn add_artist(&mut self, artist: Artist) -> Result<(), Error> {
        let conn = try!(conn());
        let stmt = try!(conn.prepare("INSERT INTO track_artists (track_id, artist_id) VALUES ($1, $2)"));
        try!(stmt.query(&[&self.id, &artist.id]));
        match self.artists {
            Some(ref mut artists) => artists.push(artist),
            None                  => self.artists = Some(vec![artist]),

        }
        Ok(())
    }

    pub fn update_with_am_song(&mut self, song: &apple_music::Song) -> &mut Track {
        self.provider      = Provider::AppleMusic;
        self.identifier    = song.id.to_string();
        self.owner_id      = Some(song.artist.to_string());
        self.owner_name    = Some(song.artist.to_string());
        self.url           = song.music_url.to_string();
        self.title         = song.title.to_string();
        self.description   = None;
        self.thumbnail_url = Some(song.artwork_url.to_string());
        self.artwork_url   = Some(song.artwork_url.to_string());
        self.audio_url     = Some(song.audio_url.to_string());
        self.state         = State::Alive;
        if let Ok(mut artist) = Artist::find_or_create(self.provider,
                                                       song.artist.to_string()) {
            artist.name  = song.artist.to_string();
            let _ = artist.save();
            let _ = self.add_artist(artist);
        }
        self
    }

    pub fn update_with_yt_video(&mut self, video: &youtube::Video) -> &mut Track {
        let s              = &video.snippet;
        self.provider      = Provider::YouTube;
        self.identifier    = video.id.to_string();
        self.owner_id      = Some(s.channelId.to_string());
        self.owner_name    = Some(s.channelTitle.to_string());
        self.url           = format!("https://www.youtube.com/watch/?v={}", video.id);
        self.title         = s.title.to_string();
        self.description   = Some(s.description.to_string());
        self.thumbnail_url = s.get_thumbnail_url();
        self.artwork_url   = s.get_artwork_url();
        self.audio_url     = None;
        self.state         = State::Alive;
        match DateTime::parse_from_rfc3339(&s.publishedAt) {
            Ok(published_at) => self.published_at = published_at.naive_utc(),
            Err(_)           => (),
        }
        if let Ok(mut artist) = Artist::find_or_create(self.provider, s.channelId.to_string()) {
            artist.name  = s.channelTitle.to_string();
            let _ = artist.save();
            let _ = self.add_artist(artist);
        }
        self
    }

    pub fn update_with_yt_playlist_item(&mut self, item: &youtube::PlaylistItem) -> &mut Track {
        let s              = &item.snippet;
        self.provider      = Provider::YouTube;
        self.identifier    = s.resourceId["videoId"].to_string();
        self.owner_id      = Some(s.channelId.to_string());
        self.owner_name    = Some(s.channelTitle.to_string());
        self.url           = format!("https://www.youtube.com/watch/?v={}", item.id);
        self.title         = s.title.to_string();
        self.description   = Some(s.description.to_string());
        self.thumbnail_url = s.get_thumbnail_url();
        self.artwork_url   = s.get_artwork_url();
        self.audio_url     = None;
        self.state         = State::Alive;
        match DateTime::parse_from_rfc3339(&s.publishedAt) {
            Ok(published_at) => self.published_at = published_at.naive_utc(),
            Err(_)           => (),
        }
        if let Ok(mut artist) = Artist::find_or_create(self.provider, s.channelId.to_string()) {
            artist.name  = s.channelTitle.to_string();
            let _ = artist.save();
            let _ = self.add_artist(artist);
        }
        self
    }


    pub fn update_with_sc_track(&mut self, track: &soundcloud::Track) -> &mut Track {
        self.provider      = Provider::SoundCloud;
        self.identifier    = track.id.to_string();
        self.owner_id      = Some(track.user.id.to_string());
        self.owner_name    = Some(track.user.username.clone());
        self.url           = track.permalink_url.to_string();
        self.title         = track.title.to_string();
        self.description   = Some(track.description.to_string());
        self.thumbnail_url = track.artwork_url.clone();
        self.artwork_url   = track.artwork_url.clone();
        self.audio_url     = Some(track.stream_url.clone());
        self.state         = State::Alive;
        match DateTime::parse_from_str(&track.created_at, "%Y/%m/%d %H:%M:%S %z") {
            Ok(published_at) => self.published_at = published_at.naive_utc(),
            Err(_)           => (),
        }
        if let Ok(mut artist) = Artist::find_or_create(self.provider,
                                                       track.user.id.to_string()) {
            artist.update_with_sc_user(&track.user);
            let _ = artist.save();
            let _ = self.add_artist(artist);
        }
        self
    }

    pub fn update_with_sp_track(&mut self, track: &spotify::Track) -> &mut Track {
        self.provider       = Provider::Spotify;
        self.identifier     = track.id.to_string();
        if track.artists.len() > 0 {
            self.owner_id   = Some(track.artists[0].id.clone());
            self.owner_name = Some(track.artists[0].name.clone());
        }
        self.url            = track.uri.clone();
        self.title          = track.name.clone();
        self.description    = None;
        self.audio_url      = track.preview_url.clone();
        self.state          = State::Alive;
        self.published_at   = UTC::now().naive_utc();
        if let Some(album) = track.album.clone() {
            self.update_with_sp_album(&album);
        }
        let artists = track.artists
            .iter()
            .map(|artist| {
                Artist::find_or_create(self.provider, artist.id.to_string())
                    .map(|mut a| a.update_with_sp_artist(&artist).clone())
            })
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect::<Vec<Artist>>();
        for mut a in artists {
            let _ = a.save();
            let _ = self.add_artist(a);
        }
        self
    }

    pub fn update_with_sp_album(&mut self, album: &spotify::Album) -> &mut Track {
        if album.images.len() > 0 {
            self.artwork_url   = Some(album.images[0].url.clone());
            self.thumbnail_url = Some(album.images[0].url.clone());
        }
        if album.images.len() > 1 {
            self.thumbnail_url = Some(album.images[1].url.clone());
        }
        self
    }

    pub fn disable(&mut self) -> &mut Track {
        self.state = State::Dead;
        self
    }
}
