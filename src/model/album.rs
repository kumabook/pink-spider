use postgres;
use uuid::Uuid;
use std::fmt;
use chrono::{NaiveDateTime, Utc};

use apple_music;
use spotify;
use error::Error;
use super::{conn, Model};
use model::provider::Provider;
use model::state::State;
use model::enclosure::Enclosure;
use model::track::Track;
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub tracks:        Vec<Track>,
    pub artists:       Option<Vec<Artist>>,
}

impl PartialEq for Album {
    fn eq(&self, p: &Album) -> bool {
        return self.identifier == p.identifier && self.provider == p.provider
    }
}

impl fmt::Display for Album {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.provider, self.identifier)
    }
}

impl<'a> Model<'a> for Album {
    fn table_name() -> String { "albums".to_string() }
    fn props_str(prefix: &str) -> String {
        PROPS
            .iter()
            .map(|&p| format!("{}{}", prefix, p))
            .collect::<Vec<String>>().join(",")
    }
    fn row_to_item(row: postgres::rows::Row) -> Album {
        Album {
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
            tracks:        vec![],
            artists:       None,
        }
    }
    fn create(&self) -> Result<Album, Error> {
        let conn = conn()?;
        let stmt = conn.prepare("INSERT INTO albums (provider, identifier, url, title)
                                      VALUES ($1, $2, $3, $4) RETURNING id")?;
        let rows = stmt.query(&[&self.provider.to_string(), &self.identifier, &self.url, &self.title])?;
        let mut album = self.clone();
        for row in rows.iter() {
            album.id = row.get(0);
        }
        Ok(album)
    }

    fn save(&mut self) -> Result<(), Error> {
        self.updated_at = Utc::now().naive_utc();
        let conn = conn()?;
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
                                      WHERE id = $1")?;
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

    fn set_relations(albums: &mut Vec<Album>) -> Result<(), Error> {
        let ids: Vec<Uuid> = albums.iter().map(|i| i.id).collect();
        let tracks_of_album = Track::find_by_albums(&ids)?;
        let artists_of_album = Artist::find_by_albums(&ids)?;
        for album in albums {
            if let Some(ref mut tracks) = tracks_of_album.get(&album.id) {
                album.tracks = tracks.clone()
            }
            if let Some(ref mut artists) = artists_of_album.get(&album.id) {
                album.artists = Some(artists.clone())
            }
        }
        Ok(())
    }
}

impl<'a> Enclosure<'a> for Album {
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
            published_at:  Utc::now().naive_utc(),
            created_at:    Utc::now().naive_utc(),
            updated_at:    Utc::now().naive_utc(),
            state:         State::Alive,
            tracks:        vec![],
            artists:       None,
        }
    }
    fn set_url(&mut self, url: String) -> &mut Album {
        self.url = url;
        self
    }
    fn set_owner_id(&mut self, owner_id: Option<String>) -> &mut Album {
        self.owner_id = owner_id;
        self
    }
    fn fetch_props(&mut self) -> Result<(), Error> {
        match self.provider {
            Provider::AppleMusic => {
                let country = apple_music::country(&self.url);
                match apple_music::fetch_album(&self.identifier, &country) {
                    Ok(album) => self.update_with_am_album(&album),
                    Err(_)    => self.disable(),
                }
            }
            Provider::Spotify => match spotify::fetch_album(&self.identifier) {
                Ok(album) => self.update_with_sp_album(&album),
                Err(_)    => self.disable(),
            },
            _ => self,
        };
        match self.state {
            State::Alive => Ok(()),
            State::Dead  => Err(Error::NotFound),
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
    pub fn find_all() -> Vec<Album> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(
            &format!("SELECT {} FROM albums ORDER BY albums.published_at DESC",
                     Album::props_str(""))).unwrap();
        let rows = stmt.query(&[]).unwrap();
        Album::rows_to_items(rows)
    }
    fn add_track(&mut self, track: &Track) -> Result<(), Error> {
        let conn = conn()?;
        let stmt = conn.prepare("INSERT INTO album_tracks (track_id, album_id)
                                 VALUES ($1, $2)")?;
        stmt.query(&[&track.id, &self.id])?;
        Ok(())
    }
    fn add_artist(&mut self, artist: &Artist) -> Result<(), Error> {
        let conn = conn()?;
        let stmt = conn.prepare("INSERT INTO album_artists (album_id, artist_id) VALUES ($1, $2)")?;
        stmt.query(&[&self.id, &artist.id])?;
        match self.artists {
            Some(ref mut artists) => artists.push(artist.clone()),
            None                  => self.artists = Some(vec![artist.clone()]),

        }
        Ok(())
    }
    pub fn find_by_artist(artist_id: Uuid) -> Vec<Album> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(
            &format!("SELECT {} FROM albums
                      LEFT OUTER JOIN album_artists ON album_artists.album_id = albums.id
                      WHERE album_artists.artist_id = $1 ORDER BY albums.created_at DESC",
                     Album::props_str("albums."))).unwrap();
        let rows = stmt.query(&[&artist_id]).unwrap();
        Album::rows_to_items(rows)
    }
    pub fn find_by_provider(provider: &Provider) -> Vec<Album> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(
            &format!("SELECT {} FROM albums WHERE albums.provider = $1
                        ORDER BY albums.created_at DESC",
                     Album::props_str(""))).unwrap();
        let rows = stmt.query(&[&(*provider).to_string()]).unwrap();
        Album::rows_to_items(rows)
    }
    pub fn from_sp_album(album: &spotify::Album) -> Album {
        Album::find_or_create(Provider::Spotify, (*album).id.to_string())
            .unwrap()
            .update_with_sp_album(album)
            .clone()
    }

    pub fn from_am_album(album: &apple_music::Album) -> Album {
        Album::find_or_create(Provider::AppleMusic, (*album).id.to_string())
            .unwrap()
            .update_with_am_album(album)
            .clone()
    }

    fn add_tracks(&mut self, tracks: Vec<Track>) {
        self.tracks = tracks.iter().map(|t| {
            let mut t = t.clone();
            if let Ok(new_track) = Track::find_or_create(t.provider,
                                                         t.identifier.to_string()) {
                t.id      = new_track.id;
                let _     = t.save();
                let _     = self.add_track(&t);
            };
            t
        }).collect::<Vec<_>>();
    }

    fn add_artists(&mut self, artists: Vec<Artist>) {
        self.artists = Some(artists.iter().map(|a| {
            let mut a = a.clone();
            if let Ok(new_artist) = Artist::find_or_create(a.provider,
                                                           a.identifier.to_string()) {
                a.id      = new_artist.id;
                let _     = a.save();
                let _     = self.add_artist(&a);
            };
            a
        }).collect::<Vec<_>>());
    }

    pub fn update_with_sp_album(&mut self, album: &spotify::Album) -> &mut Album {
        self.provider       = Provider::Spotify;
        self.identifier     = album.id.to_string();
        if album.artists.len() > 0 {
            self.owner_id   = Some(album.artists[0].id.clone());
            self.owner_name = Some(album.artists[0].name.clone());
        }
        self.url            = album.uri.clone();
        self.title          = album.name.clone();
        self.description    = None;
        self.state          = State::Alive;
        self.published_at   = Utc::now().naive_utc();
        if album.images.len() > 0 {
            self.artwork_url   = Some(album.images[0].url.clone());
            self.thumbnail_url = Some(album.images[0].url.clone());
        }
        if album.images.len() > 1 {
            self.thumbnail_url = Some(album.images[1].url.clone());
        }

        let artist_ids = album.artists.iter().map(|a| a.id.clone()).collect::<Vec<String>>();
        let sp_artists = spotify::fetch_artists(artist_ids).unwrap_or_default();
        let artists = sp_artists.iter().map(|ref a| Artist::from_sp_artist(a))
            .collect::<Vec<_>>();
        self.add_artists(artists);

        let track_ids = album.tracks.clone()
            .map(|t| t.items).unwrap_or(vec![]).iter()
            .filter(|ref t| t.id.is_some())
            .map(|t| t.clone().id.unwrap())
            .collect();
        let sp_tracks = spotify::fetch_tracks(track_ids).unwrap_or(vec![]);
        let tracks = sp_tracks.iter()
            .map(|ref t| Track::from_sp_track(t))
            .filter(|ref t| t.is_ok())
            .map(|t| t.unwrap().clone())
            .map(|ref mut t| t.update_with_sp_album(&album).clone())
            .collect::<Vec<_>>();
        self.add_tracks(tracks);
        self
    }

    pub fn update_with_am_album(&mut self, album: &apple_music::Album) -> &mut Album {
        let album_artists = album.clone().relationships.map(|r| {
            r.artists.data.clone()
        });
        if let Some(album_artist) = album_artists.clone().and_then(|a| a.first().map(|a| a.clone())) {
            let artist_name    = album_artist.attributes.name.clone();
            self.provider      = Provider::AppleMusic;
            self.identifier    = album.id.to_string();
            self.owner_id      = Some(album_artist.id.to_string());
            self.owner_name    = Some(artist_name.to_string());
            self.url           = album.attributes.url.clone();
            self.title         = album.attributes.name.clone();
            self.description   = album.attributes.editorial_notes.clone().and_then(|n| n.short.clone());
            self.thumbnail_url = Some(album.attributes.artwork.get_thumbnail_url());
            self.artwork_url   = Some(album.attributes.artwork.get_artwork_url());
            self.state         = State::Alive;
        }
        let country = apple_music::country(&self.url);
        if let Some(album_artists) = album_artists.clone() {
            let artist_ids = album_artists.iter().map(|a| a.id.clone()).collect::<Vec<String>>();
            let artists = apple_music::fetch_artists(&country, artist_ids).unwrap_or(vec![]);
            self.add_artists(artists.iter().map(|a| Artist::from_am_artist(a)).collect());
        }
        let album_tracks = album.clone().relationships.map(|r| {
            r.tracks.data.clone()
        }).unwrap_or(vec![]);
        let songs = album_tracks.iter().map(|track| match *track {
            apple_music::Track::Song(ref song) => Some(song),
            apple_music::Track::MusicVideo(_) => None,
        }).filter(|song| song.is_some())
            .map(|song| song.unwrap())
            .collect::<Vec<_>>();
        let song_ids = songs.iter().map(|song| song.id.clone()).collect::<Vec<String>>();
        let songs = apple_music::fetch_songs(&country, song_ids).unwrap_or(vec![]);
        self.add_tracks(songs.iter().map(|song| Track::from_am_song(song)).collect());
        self
    }

    pub fn disable(&mut self) -> &mut Album {
        self.state = State::Dead;
        self
    }

    pub fn delete(&self) -> Result<(), Error> {
        let conn = conn()?;
        let stmt = conn.prepare("DELETE FROM albums WHERE id=$1")?;
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
