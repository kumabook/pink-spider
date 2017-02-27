use std::collections::BTreeMap;
use rustc_serialize::json::{ToJson, Json};
use postgres;
use uuid::Uuid;
use std::fmt;
use chrono::{NaiveDateTime, UTC, DateTime};

use soundcloud;
use spotify;
use error::Error;
use super::{conn, Model};
use model::provider::Provider;

static PROPS: [&'static str; 9]  = ["id",
                                    "provider",
                                    "identifier",
                                    "url",
                                    "name",
                                    "thumbnail_url",
                                    "artwork_url",
                                    "created_at",
                                    "updated_at"];

#[derive(Debug, Clone)]
pub struct Artist {
    pub id:            Uuid,
    pub provider:      Provider,
    pub identifier:    String,
    pub url:           String,
    pub name:          String,
    pub thumbnail_url: Option<String>,
    pub artwork_url:   Option<String>,
    pub created_at:    NaiveDateTime,
    pub updated_at:    NaiveDateTime,
}

impl ToJson for Artist {
    fn to_json(&self) -> Json {
        let created_at   = DateTime::<UTC>::from_utc(self.created_at  , UTC);
        let updated_at   = DateTime::<UTC>::from_utc(self.updated_at  , UTC);
        let mut d = BTreeMap::new();
        d.insert("id".to_string()           , self.id.to_string().to_json());
        d.insert("provider".to_string()     , self.provider.to_json());
        d.insert("identifier".to_string()   , self.identifier.to_json());
        d.insert("url".to_string()          , self.url.to_json());
        d.insert("name".to_string()         , self.name.to_json());
        d.insert("thumbnail_url".to_string(), self.thumbnail_url.to_json());
        d.insert("artwork_url".to_string()  , self.artwork_url.to_json());
        d.insert("created_at".to_string()   , created_at.to_rfc3339().to_json());
        d.insert("updated_at".to_string()   , updated_at.to_rfc3339().to_json());
        Json::Object(d)
    }
}

impl fmt::Display for Artist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.provider, self.identifier)
    }
}

impl Model for Artist {
    fn table_name() -> String { "artists".to_string() }
    fn props_str(prefix: &str) -> String {
        PROPS
            .iter()
            .map(|&p| format!("{}{}", prefix, p))
            .collect::<Vec<String>>().join(",")
    }
    fn rows_to_items(rows: postgres::rows::Rows) -> Vec<Artist> {
        let mut artists = Vec::new();
        for row in rows.iter() {
            let artist = Artist {
                id:            row.get(0),
                provider:      Provider::new(row.get(1)),
                identifier:    row.get(2),
                url:           row.get(3),
                name:          row.get(4),
                thumbnail_url: row.get(5),
                artwork_url:   row.get(6),
                created_at:    row.get(7),
                updated_at:    row.get(8),
            };
            artists.push(artist)
        }
        artists
    }

    fn create(&self) -> Result<Artist, Error> {
        let conn = try!(conn());
        let stmt = try!(conn.prepare("INSERT INTO artists (provider, identifier, url, name)
                                 VALUES ($1, $2, $3, $4) RETURNING id"));
        let rows = try!(stmt.query(&[&self.provider.to_string(), &self.identifier, &self.url, &self.name]));
        let mut artist = self.clone();
        for row in rows.iter() {
            artist.id = row.get(0);
        }
        Ok(artist)
    }

    fn save(&mut self) -> Result<(), Error> {
        self.updated_at = UTC::now().naive_utc();
        let conn = conn().unwrap();
        let stmt = conn.prepare("UPDATE artists SET
                                 provider      = $2,
                                 identifier    = $3,
                                 url           = $4,
                                 name          = $5,
                                 thumbnail_url = $6,
                                 artwork_url   = $7,
                                 created_at    = $8,
                                 updated_at    = $9
                                 WHERE id = $1").unwrap();
        let result = stmt.query(&[&self.id,
                                  &self.provider.to_string(),
                                  &self.identifier,
                                  &self.url,
                                  &self.name,
                                  &self.thumbnail_url,
                                  &self.artwork_url,
                                  &self.created_at,
                                  &self.updated_at,
        ]);
        match result {
            Ok(_)  => Ok(()),
            Err(_) => Err(Error::Unexpected),
        }
    }
}

impl Artist {
    fn new(provider: Provider, identifier: String) -> Artist {
        Artist {
            id:            Uuid::new_v4(),
            provider:      provider,
            identifier:    identifier,
            url:           "".to_string(),
            name:          "".to_string(),
            thumbnail_url: None,
            artwork_url:   None,
            created_at:    UTC::now().naive_utc(),
            updated_at:    UTC::now().naive_utc(),
        }
    }
    fn find_by(provider: &Provider, identifier: &str) -> Result<Artist, Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(
            &format!("SELECT {} FROM {}
                     WHERE provider = $1 AND identifier = $2
                     ORDER BY updated_at DESC",
                     Artist::props_str(""), Artist::table_name())).unwrap();
        let rows = stmt.query(&[&(*provider).to_string(), &identifier]).unwrap();
        let items = Artist::rows_to_items(rows);
        if items.len() > 0 {
            return Ok(items[0].clone());
        }
        return Err(Error::NotFound)
    }
    pub fn find_or_create(provider: Provider, identifier: String) -> Result<Artist, Error> {
        return match Artist::find_by(&provider, &identifier) {
            Ok(item) => Ok(item),
            Err(_)   => Artist::new(provider, identifier).create()
        }
    }
    pub fn from_yt_channel(channel_id: &str, channel_title: &str) -> Artist {
        let mut artist = Artist::new(Provider::YouTube, channel_id.to_string());
        artist.name = channel_title.to_string();
        artist.clone()
    }
    pub fn from_sp_artist(artist: &spotify::Artist) -> Artist {
        Artist::new(Provider::Spotify, (*artist).id.to_string())
            .update_with_sp_artist(artist)
            .clone()
    }

    pub fn from_sc_user(playlist: &soundcloud::User) -> Artist {
        Artist::new(Provider::SoundCloud, (*playlist).id.to_string())
            .update_with_sc_user(playlist)
            .clone()
    }

    pub fn from_am_artist(artist: &str) -> Artist {
        let mut artist = Artist::new(Provider::AppleMusic, artist.to_string());
        artist.name = artist.to_string();
        artist.clone()
    }

    pub fn update_with_sp_artist(&mut self, artist: &spotify::Artist) -> &mut Artist {
        self.provider       = Provider::Spotify;
        self.identifier     = artist.id.to_string();
        self.url            = artist.uri.clone();
        self.name           = artist.name.clone();
        if let Some(ref images) = artist.images {
            if images.len() > 0 {
                self.artwork_url   = Some(images[0].url.clone());
                self.thumbnail_url = Some(images[0].url.clone());
            }
            if images.len() > 1 {
                self.thumbnail_url = Some(images[1].url.clone());
            }
        }
        self
    }

    pub fn update_with_sc_user(&mut self, user: &soundcloud::User) -> &mut Artist {
        self.provider      = Provider::SoundCloud;
        self.identifier    = user.id.to_string();
        self.url           = user.permalink.to_string();
        self.name          = user.username.to_string();
        self.thumbnail_url = Some(user.avatar_url.clone());
        self.artwork_url   = Some(user.avatar_url.clone());
        self
    }
}

#[cfg(test)]
mod test {
    use model::Model;
    use super::Artist;
    use Provider;
    #[test]
    fn test_new() {
        let artist = Artist::new(Provider::Spotify,
                               "4tZwfgrHOc3mvqYlEYSvVi".to_string());
        assert_eq!(artist.provider, Provider::Spotify);
        assert_eq!(&artist.identifier, "4tZwfgrHOc3mvqYlEYSvVi");
    }
    #[test]
    fn test_find_of_create() {
        let identifier = "4tZwfgrHOc3mvqYlEYSvVi".to_string();
        let result     = Artist::find_or_create(Provider::Spotify, identifier);
        assert!(result.is_ok());
    }
    #[test]
    fn test_save() {
        let id = "test_save";
        let mut artist = Artist::find_or_create(Provider::Spotify, id.to_string()).unwrap();
        artist.name   = "name".to_string();
        let result    = artist.save();
        assert!(result.is_ok());
        let artist = Artist::find_or_create(Provider::Spotify, id.to_string()).unwrap();
        assert_eq!(&artist.name, "name");
    }
}
