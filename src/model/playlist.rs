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
use super::{conn, PaginatedCollection};
use model::provider::Provider;
use model::state::State;

static PROPS: [&'static str; 12]  = ["id",
                                     "provider",
                                     "identifier",
                                     "url",
                                     "title",
                                     "description",
                                     "thumbnail_url",
                                     "artwork_url",
                                     "published_at",
                                     "created_at",
                                     "updated_at",
                                     "state"];

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Playlist {
    pub id:            Uuid,
    pub provider:      Provider,
    pub identifier:    String,
    pub url:           String,
    pub title:         String,
    pub description:   Option<String>,
    pub thumbnail_url: Option<String>,
    pub artwork_url:   Option<String>,
    pub published_at:  NaiveDateTime,
    pub created_at:    NaiveDateTime,
    pub updated_at:    NaiveDateTime,
    pub state:         State,
}

fn props_str(prefix: &str) -> String {
    PROPS
        .iter()
        .map(|&p| format!("{}{}", prefix, p))
        .collect::<Vec<String>>().join(",")
}

impl PartialEq for Playlist {
    fn eq(&self, p: &Playlist) -> bool {
        return self.identifier == p.identifier && self.provider == p.provider
    }
}

impl ToJson for Playlist {
    fn to_json(&self) -> Json {
        let published_at = DateTime::<UTC>::from_utc(self.published_at, UTC);
        let created_at   = DateTime::<UTC>::from_utc(self.created_at  , UTC);
        let updated_at   = DateTime::<UTC>::from_utc(self.updated_at  , UTC);
        let mut d = BTreeMap::new();
        d.insert("id".to_string()           , self.id.to_string().to_json());
        d.insert("provider".to_string()     , self.provider.to_json());
        d.insert("identifier".to_string()   , self.identifier.to_json());
        d.insert("url".to_string()          , self.url.to_json());
        d.insert("title".to_string()        , self.title.to_json());
        d.insert("description".to_string()  , self.description.to_json());
        d.insert("thumbnail_url".to_string(), self.thumbnail_url.to_json());
        d.insert("artwork_url".to_string()  , self.artwork_url.to_json());
        d.insert("published_at".to_string() , published_at.to_rfc3339().to_json());
        d.insert("created_at".to_string()   , created_at.to_rfc3339().to_json());
        d.insert("updated_at".to_string()   , updated_at.to_rfc3339().to_json());
        d.insert("state".to_string()        , self.state.to_json());
        Json::Object(d)
    }
}

impl fmt::Display for Playlist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.provider, self.identifier)
    }
}

impl Playlist {
    pub fn new(provider: Provider, identifier: String) -> Playlist {
        Playlist {
            id:            Uuid::new_v4(),
            provider:      provider,
            identifier:    identifier,
            url:           "".to_string(),
            title:         "".to_string(),
            description:   None,
            thumbnail_url: None,
            artwork_url:   None,
            published_at:  UTC::now().naive_utc(),
            created_at:    UTC::now().naive_utc(),
            updated_at:    UTC::now().naive_utc(),
            state:         State::Alive,
        }
    }

    pub fn from_yt_playlist(playlist: &youtube::Playlist) -> Playlist {
        Playlist::new(Provider::YouTube, (*playlist).id.to_string())
            .update_with_yt_playlist(playlist)
            .clone()
    }

    pub fn from_sp_playlist(playlist: &spotify::Playlist) -> Playlist {
        Playlist::new(Provider::Spotify, (*playlist).id.to_string())
            .update_with_sp_playlist(playlist)
            .clone()
    }

    pub fn from_sc_playlist(playlist: &soundcloud::Playlist) -> Playlist {
        Playlist::new(Provider::SoundCloud, (*playlist).id.to_string())
            .update_with_sc_playlist(playlist)
            .clone()
    }

    pub fn from_am_playlist(playlist: &apple_music::Playlist) -> Playlist {
        Playlist::new(Provider::AppleMusic, (*playlist).id.to_string())
            .update_with_am_playlist(playlist)
            .clone()
    }

    pub fn update_with_yt_playlist(&mut self, playlist: &youtube::Playlist) -> &mut Playlist {
        self.provider      = Provider::YouTube;
        self.identifier    = playlist.id.to_string();
        self.url           = format!("https://www.youtube.com/watch/?v={}", playlist.id);
        self.title         = playlist.snippet.title.to_string();
        self.description   = Some(playlist.snippet.description.to_string());
        self.thumbnail_url = playlist.snippet.get_thumbnail_url();
        self.artwork_url   = playlist.snippet.get_artwork_url();
        self.state         = State::Alive;
        match DateTime::parse_from_rfc3339(&playlist.snippet.publishedAt) {
            Ok(published_at) => self.published_at = published_at.naive_utc(),
            Err(_)           => (),
        }
        self
    }

    pub fn update_with_sp_playlist(&mut self, playlist: &spotify::Playlist) -> &mut Playlist {
        self.provider       = Provider::Spotify;
        self.identifier     = playlist.id.to_string();
        self.url            = playlist.uri.clone();
        self.title          = playlist.name.clone();
        self.description    = None;
        self.state          = State::Alive;
        self.published_at   = UTC::now().naive_utc();
        if playlist.images.len() > 0 {
            self.artwork_url   = Some(playlist.images[0].url.clone());
            self.thumbnail_url = Some(playlist.images[0].url.clone());
        }
        if playlist.images.len() > 1 {
            self.thumbnail_url = Some(playlist.images[1].url.clone());
        }
        self
    }

    pub fn update_with_sc_playlist(&mut self, playlist: &soundcloud::Playlist) -> &mut Playlist {
        self.provider      = Provider::SoundCloud;
        self.identifier    = playlist.id.to_string();
        self.url           = playlist.permalink_url.to_string();
        self.title         = playlist.title.to_string();
        self.description   = None;
        self.thumbnail_url = playlist.artwork_url.clone();
        self.artwork_url   = playlist.artwork_url.clone();
        self.state         = State::Alive;
        match DateTime::parse_from_str(&playlist.created_at, "%Y/%m/%d %H:%M:%S %z") {
            Ok(published_at) => self.published_at = published_at.naive_utc(),
            Err(_)           => (),
        }
        self
    }

    pub fn update_with_am_playlist(&mut self, playlist: &apple_music::Playlist) -> &mut Playlist {
        self.provider      = Provider::AppleMusic;
        self.identifier    = playlist.id.to_string();
        self.url           = playlist.music_url.to_string();
        self.title         = playlist.title.to_string();
        self.description   = Some(playlist.description.to_string());
        self.thumbnail_url = Some(playlist.artwork_url.to_string());
        self.artwork_url   = Some(playlist.artwork_url.to_string());
        self.state         = State::Alive;
        self
    }

    pub fn disable(&mut self) -> &mut Playlist {
        self.state = State::Dead;
        self
    }

    fn rows_to_playlists(rows: postgres::rows::Rows) -> Vec<Playlist> {
        let mut playlists = Vec::new();
        for row in rows.iter() {
            let playlist = Playlist {
                id:            row.get(0),
                provider:      Provider::new(row.get(1)),
                identifier:    row.get(2),
                url:           row.get(3),
                title:         row.get(4),
                description:   row.get(5),
                thumbnail_url: row.get(6),
                artwork_url:   row.get(7),
                published_at:  row.get(8),
                created_at:    row.get(9),
                updated_at:    row.get(10),
                state:         State::new(row.get(11)),
            };
            playlists.push(playlist)
        }
        playlists
    }


    pub fn find_by_id(id: &str) -> Result<Playlist, Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(
            &format!("SELECT {} FROM playlists
                        WHERE id = $1", props_str(""))).unwrap();
        let uuid      = try!(Uuid::parse_str(id).map_err(|_| Error::Unprocessable));
        let rows      = stmt.query(&[&uuid]).unwrap();
        let playlists = Playlist::rows_to_playlists(rows);
        if playlists.len() > 0 {
            return Ok(playlists[0].clone());
        }
        return Err(Error::NotFound)
    }

    pub fn find_by(provider: &Provider, identifier: &str) -> Result<Playlist, Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(
            &format!("SELECT {} FROM playlists
                     WHERE provider = $1 AND identifier = $2
                     ORDER BY published_at DESC", props_str(""))).unwrap();
        let rows      = stmt.query(&[&(*provider).to_string(), &identifier]).unwrap();
        let playlists = Playlist::rows_to_playlists(rows);
        if playlists.len() > 0 {
            return Ok(playlists[0].clone());
        }
        return Err(Error::NotFound)
    }

    pub fn find_by_entry_id(entry_id: Uuid) -> Vec<Playlist> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(
            &format!("SELECT {} FROM playlists p LEFT JOIN playlist_entries pe
                        ON p.id = pe.playlist_id
                        WHERE pe.entry_id = $1
                        ORDER BY p.published_at DESC", props_str("p."))).unwrap();
        let rows = stmt.query(&[&entry_id]).unwrap();
        Playlist::rows_to_playlists(rows)
    }

    pub fn find(page: i64, per_page: i64) -> PaginatedCollection<Playlist> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(&format!("SELECT {}  FROM playlists
                                            ORDER BY published_at DESC
                                            LIMIT $2 OFFSET $1", props_str(""))).unwrap();
        let offset    = page * per_page;
        let rows      = stmt.query(&[&offset, &per_page]).unwrap();
        let playlists = Playlist::rows_to_playlists(rows);
        let mut total: i64 = 0;
        for row in conn.query("SELECT COUNT(*) FROM playlists", &[]).unwrap().iter() {
            total = row.get(0);
        }
        PaginatedCollection {
            page:     page,
            per_page: per_page,
            total:    total,
            items:    playlists,
        }
    }

    pub fn create(&self) -> Result<Playlist, Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("INSERT INTO playlists (provider, identifier, url, title)
                                 VALUES ($1, $2, $3, $4) RETURNING id").unwrap();
        let rows = try!(stmt.query(&[&self.provider.to_string(), &self.identifier, &self.url, &self.title]));
        let mut playlist = self.clone();
        for row in rows.iter() {
            playlist.id = row.get(0);
        }
        Ok(playlist)
    }

    pub fn find_or_create(provider: Provider, identifier: String) -> Result<Playlist, Error> {
        return match Playlist::find_by(&provider, &identifier) {
            Ok(playlist) => Ok(playlist),
            Err(_)    => Playlist::new(provider, identifier).create()
        }
    }

    pub fn save(&mut self) -> Result<(), Error> {
        self.updated_at = UTC::now().naive_utc();
        let conn = conn().unwrap();
        let stmt = conn.prepare("UPDATE playlists SET
                                 provider      = $2,
                                 identifier    = $3,
                                 url           = $4,
                                 title         = $5,
                                 description   = $6,
                                 thumbnail_url = $7,
                                 artwork_url   = $8,
                                 published_at  = $9,
                                 created_at    = $10,
                                 updated_at    = $11,
                                 state         = $12
                                 WHERE id = $1").unwrap();
        let result = stmt.query(&[&self.id,
                                  &self.provider.to_string(),
                                  &self.identifier,
                                  &self.url,
                                  &self.title,
                                  &self.description,
                                  &self.thumbnail_url,
                                  &self.artwork_url,
                                  &self.published_at,
                                  &self.created_at,
                                  &self.updated_at,
                                  &self.state.to_string(),
        ]);
        match result {
            Ok(_)  => Ok(()),
            Err(_) => Err(Error::Unexpected),
        }
    }

    pub fn delete(&self) -> Result<(), Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("DELETE FROM playlists WHERE id=$1").unwrap();
        let result = stmt.query(&[&self.id]);
        match result {
            Ok(_)  => Ok(()),
            Err(_) => Err(Error::Unexpected)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Playlist;
    use Provider;
    #[test]
    fn test_new() {
        let playlist = Playlist::new(Provider::YouTube,
                                     "PLy8LZ8FM-o0ViuGAF68RAaXkQ8V-3dbTX".to_string());
        assert_eq!(playlist.provider, Provider::YouTube);
        assert_eq!(&playlist.identifier, "PLy8LZ8FM-o0ViuGAF68RAaXkQ8V-3dbTX")
    }
    #[test]
    fn test_find_of_create() {
        let identifier = "PLy8LZ8FM-o0ViuGAF68RAaXkQ8V-3dbTX".to_string();
        let result     = Playlist::find_or_create(Provider::YouTube, identifier);
        assert!(result.is_ok());
    }
    #[test]
    fn test_delete() {
        let identifier = "test_delete".to_string();
        let playlist   = Playlist::find_or_create(Provider::YouTube, identifier).unwrap();
        let result     = playlist.delete();
        assert!(result.is_ok());
    }
    #[test]
    fn test_save() {
        let id = "test_save";
        let mut playlist = Playlist::find_or_create(Provider::YouTube, id.to_string()).unwrap();
        playlist.title = "title".to_string();
        let result = playlist.save();
        assert!(result.is_ok());
        let playlist = Playlist::find_or_create(Provider::YouTube, id.to_string()).unwrap();
        assert_eq!(&playlist.title, "title");
    }
}
