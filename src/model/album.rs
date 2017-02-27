use std::collections::BTreeMap;
use rustc_serialize::json::{ToJson, Json};
use postgres;
use uuid::Uuid;
use std::fmt;
use chrono::{NaiveDateTime, UTC, DateTime};

use apple_music;
use spotify;
use error::Error;
use super::{conn, Model};
use model::provider::Provider;
use model::state::State;
use model::enclosure::Enclosure;
use model::artist::Artist;

static PROPS: [&'static str; 14]  = ["id",
                                     "provider",
                                     "identifier",
                                     "owner_id",
                                     "owner_name",
                                     "url",
                                     "title",
                                     "description",
                                     "thumbnail_url",
                                     "artwork_url",
                                     "published_at",
                                     "created_at",
                                     "updated_at",
                                     "state"];

#[derive(Debug, Clone)]
pub struct Album {
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
    pub published_at:  NaiveDateTime,
    pub created_at:    NaiveDateTime,
    pub updated_at:    NaiveDateTime,
    pub state:         State,
    pub artists:       Option<Vec<Artist>>,
}

impl PartialEq for Album {
    fn eq(&self, p: &Album) -> bool {
        return self.identifier == p.identifier && self.provider == p.provider
    }
}

impl ToJson for Album {
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
        d.insert("artists".to_string()      , self.artists.to_json());
        Json::Object(d)
    }
}

impl fmt::Display for Album {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.provider, self.identifier)
    }
}

impl Model for Album {
    fn table_name() -> String { "albums".to_string() }
    fn props_str(prefix: &str) -> String {
        PROPS
            .iter()
            .map(|&p| format!("{}{}", prefix, p))
            .collect::<Vec<String>>().join(",")
    }

    fn rows_to_items(rows: postgres::rows::Rows) -> Vec<Album> {
        let mut albums = Vec::new();
        for row in rows.iter() {
            let album = Album {
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
                published_at:  row.get(10),
                created_at:    row.get(11),
                updated_at:    row.get(12),
                state:         State::new(row.get(13)),
                artists:       None,
            };
            albums.push(album)
        }
        albums
    }

    fn create(&self) -> Result<Album, Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("INSERT INTO albums (provider, identifier, url, title)
                                 VALUES ($1, $2, $3, $4) RETURNING id").unwrap();
        let rows = try!(stmt.query(&[&self.provider.to_string(), &self.identifier, &self.url, &self.title]));
        let mut album = self.clone();
        for row in rows.iter() {
            album.id = row.get(0);
        }
        Ok(album)
    }

    fn save(&mut self) -> Result<(), Error> {
        self.updated_at = UTC::now().naive_utc();
        let conn = conn().unwrap();
        let stmt = conn.prepare("UPDATE albums SET
                                 provider      = $2,
                                 identifier    = $3,
                                 owner_id      = $4,
                                 owner_name    = $5,
                                 url           = $6,
                                 title         = $7,
                                 description   = $8,
                                 thumbnail_url = $9,
                                 artwork_url   = $10,
                                 published_at  = $11,
                                 created_at    = $12,
                                 updated_at    = $13,
                                 state         = $14
                                 WHERE id = $1").unwrap();
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
}

impl Enclosure for Album {
    fn new(provider: Provider, identifier: String) -> Album {
        Album {
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
            published_at:  UTC::now().naive_utc(),
            created_at:    UTC::now().naive_utc(),
            updated_at:    UTC::now().naive_utc(),
            state:         State::Alive,
            artists:       None,
        }
    }
    fn find_by_entry_id(entry_id: Uuid) -> Vec<Album> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(
            &format!("SELECT {} FROM albums p LEFT JOIN album_entries pe
                        ON p.id = pe.album_id
                        WHERE pe.entry_id = $1
                        ORDER BY p.published_at DESC",
                     Album::props_str("p."))).unwrap();
        let rows = stmt.query(&[&entry_id]).unwrap();
        Album::rows_to_items(rows)
    }
}

impl Album {
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

    pub fn from_sp_album(album: &spotify::Album) -> Album {
        Album::new(Provider::Spotify, (*album).id.to_string())
            .update_with_sp_album(album)
            .clone()
    }

    pub fn from_am_album(album: &apple_music::Album) -> Album {
        Album::new(Provider::AppleMusic, (*album).id.to_string())
            .update_with_am_album(album)
            .clone()
    }

    pub fn update_with_sp_album(&mut self, album: &spotify::Album) -> &mut Album {
        self.provider       = Provider::Spotify;
        self.identifier     = album.id.to_string();
        self.url            = album.uri.clone();
        self.title          = album.name.clone();
        self.description    = None;
        self.state          = State::Alive;
        self.published_at   = UTC::now().naive_utc();
        if album.images.len() > 0 {
            self.artwork_url   = Some(album.images[0].url.clone());
            self.thumbnail_url = Some(album.images[0].url.clone());
        }
        if album.images.len() > 1 {
            self.thumbnail_url = Some(album.images[1].url.clone());
        }
        let artists = album.artists
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

    pub fn update_with_am_album(&mut self, album: &apple_music::Album) -> &mut Album {
        self.provider      = Provider::AppleMusic;
        self.identifier    = album.id.to_string();
        self.url           = album.music_url.to_string();
        self.title         = album.title.to_string();
        self.description   = None;
        self.thumbnail_url = Some(album.artwork_url.to_string());
        self.artwork_url   = Some(album.artwork_url.to_string());
        self.state         = State::Alive;
        if let Ok(mut artist) = Artist::find_or_create(self.provider,
                                                       album.album_artist.to_string()) {
            artist.name  = album.album_artist.to_string();
            let _ = artist.save();
            let _ = self.add_artist(artist);
        }
        self
    }

    pub fn disable(&mut self) -> &mut Album {
        self.state = State::Dead;
        self
    }

    pub fn delete(&self) -> Result<(), Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("DELETE FROM albums WHERE id=$1").unwrap();
        let result = stmt.query(&[&self.id]);
        match result {
            Ok(_)  => Ok(()),
            Err(_) => Err(Error::Unexpected)
        }
    }
}

#[cfg(test)]
mod test {
    use model::enclosure::Enclosure;
    use model::Model;
    use super::Album;
    use Provider;
    #[test]
    fn test_new() {
        let album = Album::new(Provider::YouTube,
                               "PLy8LZ8FM-o0ViuGAF68RAaXkQ8V-3dbTX".to_string());
        assert_eq!(album.provider, Provider::YouTube);
        assert_eq!(&album.identifier, "PLy8LZ8FM-o0ViuGAF68RAaXkQ8V-3dbTX")
    }
    #[test]
    fn test_find_of_create() {
        let identifier = "PLy8LZ8FM-o0ViuGAF68RAaXkQ8V-3dbTX".to_string();
        let result     = Album::find_or_create(Provider::YouTube, identifier);
        assert!(result.is_ok());
    }
    #[test]
    fn test_delete() {
        let identifier = "test_delete".to_string();
        let album      = Album::find_or_create(Provider::YouTube, identifier).unwrap();
        let result     = album.delete();
        assert!(result.is_ok());
    }
    #[test]
    fn test_save() {
        let id = "test_save";
        let mut album = Album::find_or_create(Provider::YouTube, id.to_string()).unwrap();
        album.title   = "title".to_string();
        let result    = album.save();
        assert!(result.is_ok());
        let album = Album::find_or_create(Provider::YouTube, id.to_string()).unwrap();
        assert_eq!(&album.title, "title");
    }
}
