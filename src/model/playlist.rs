use postgres;
use uuid::Uuid;
use std::fmt;
use std::collections::BTreeMap;
use chrono::{NaiveDateTime, Utc, DateTime};
use params;

use apple_music;
use youtube;
use youtube::HasThumbnail;
use soundcloud;
use spotify;
use error::Error;
use super::{conn, Model};
use model::provider::Provider;
use model::state::State;
use model::enclosure::Enclosure;
use model::track::Track;
use model::playlist_track::PlaylistTrack;

static PROPS: [&'static str; 15]  = ["id",
                                     "provider",
                                     "identifier",
                                     "owner_id",
                                     "owner_name",
                                     "url",
                                     "title",
                                     "description",
                                     "velocity",
                                     "thumbnail_url",
                                     "artwork_url",
                                     "published_at",
                                     "created_at",
                                     "updated_at",
                                     "state"];

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Playlist {
    pub id:            Uuid,
    pub provider:      Provider,
    pub identifier:    String,
    pub owner_id:      Option<String>,
    pub owner_name:    Option<String>,
    pub url:           String,
    pub title:         String,
    pub description:   Option<String>,
    pub velocity:      f64,
    pub thumbnail_url: Option<String>,
    pub artwork_url:   Option<String>,
    pub published_at:  NaiveDateTime,
    pub created_at:    NaiveDateTime,
    pub updated_at:    NaiveDateTime,
    pub state:         State,
    pub tracks:        Vec<PlaylistTrack>,
}

impl PartialEq for Playlist {
    fn eq(&self, p: &Playlist) -> bool {
        return self.identifier == p.identifier && self.provider == p.provider
    }
}

impl fmt::Display for Playlist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.provider, self.identifier)
    }
}

impl<'a> Model<'a> for Playlist {
    fn table_name() -> String { "playlists".to_string() }
    fn props_str(prefix: &str) -> String {
        PROPS
            .iter()
            .map(|&p| format!("{}{}", prefix, p))
            .collect::<Vec<String>>().join(",")
    }
    fn row_to_item(row: postgres::rows::Row) -> Playlist {
        Playlist {
            id:            row.get(0),
            provider:      Provider::new(row.get(1)),
            identifier:    row.get(2),
            owner_id:      row.get(3),
            owner_name:    row.get(4),
            url:           row.get(5),
            title:         row.get(6),
            description:   row.get(7),
            velocity:      row.get(8),
            thumbnail_url: row.get(9),
            artwork_url:   row.get(10),
            published_at:  row.get(11),
            created_at:    row.get(12),
            updated_at:    row.get(13),
            state:         State::new(row.get(14)),
            tracks:        vec![],
        }
    }
    fn create(&self) -> Result<Playlist, Error> {
        let conn = conn()?;
        let stmt = conn.prepare("INSERT INTO playlists (provider, identifier, url, title)
                                 VALUES ($1, $2, $3, $4) RETURNING id")?;
        let rows = stmt.query(&[&self.provider.to_string(), &self.identifier, &self.url, &self.title])?;
        let mut playlist = self.clone();
        for row in rows.iter() {
            playlist.id = row.get(0);
        }
        Ok(playlist)
    }

    fn save(&mut self) -> Result<(), Error> {
        self.updated_at = Utc::now().naive_utc();
        let conn = conn()?;
        let stmt = conn.prepare("UPDATE playlists SET
                                      provider      = $2,
                                      identifier    = $3,
                                      owner_id      = $4,
                                      owner_name    = $5,
                                      url           = $6,
                                      title         = $7,
                                      description   = $8,
                                      velocity      = $9,
                                      thumbnail_url = $10,
                                      artwork_url   = $11,
                                      published_at  = $12,
                                      created_at    = $13,
                                      updated_at    = $14,
                                      state         = $15
                                      WHERE id = $1")?;
        let result = stmt.query(&[&self.id,
                                  &self.provider.to_string(),
                                  &self.identifier,
                                  &self.owner_id,
                                  &self.owner_name,
                                  &self.url,
                                  &self.title,
                                  &self.description,
                                  &self.velocity,
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

    fn set_relations(playlists: &mut Vec<Playlist>) -> Result<(), Error> {
        let ids: Vec<Uuid> = playlists.iter().map(|i| i.id).collect();
        let playlist_tracks_map = PlaylistTrack::find_by_playlist_ids(ids.clone())?;
        for playlist in playlists {
            if let Some(playlist_tracks) = playlist_tracks_map.get(&playlist.id) {
                playlist.tracks = playlist_tracks.clone()
            }
        }
        Ok(())
    }

    fn update_attributes(&mut self, map: &params::Map) -> &mut Self {
        match map.find(&["velocity"]) {
            Some(&params::Value::F64(ref value)) => self.velocity = *value,
            Some(&params::Value::I64(ref value)) => self.velocity = *value as f64,
            Some(&params::Value::U64(ref value)) => self.velocity = *value as f64,
            Some(&params::Value::String(ref v)) => if let Ok(v) = (*v).parse::<f64>() {
                self.velocity = v.clone()
            },
            _                                    => (),
        };
        self
    }
}

impl<'a> Enclosure<'a> for Playlist {
    fn new(provider: Provider, identifier: String) -> Playlist {
        Playlist {
            id:            Uuid::new_v4(),
            provider:      provider,
            identifier:    identifier,
            owner_id:      None,
            owner_name:    None,
            url:           "".to_string(),
            title:         "".to_string(),
            description:   None,
            velocity:      0.0,
            thumbnail_url: None,
            artwork_url:   None,
            published_at:  Utc::now().naive_utc(),
            created_at:    Utc::now().naive_utc(),
            updated_at:    Utc::now().naive_utc(),
            state:         State::Alive,
            tracks:        vec![],
        }
    }
    fn set_url(&mut self, url: String) -> &mut Playlist {
        self.url = url;
        self
    }
    fn set_owner_id(&mut self, owner_id: Option<String>) -> &mut Playlist {
        self.owner_id = owner_id;
        self
    }
    fn fetch_props(&mut self) -> Result<(), Error> {
        match self.provider {
            Provider::YouTube => {
                let items = youtube::fetch_playlist_items(&self.identifier)
                    .map(|res| res.items)
                    .unwrap_or(vec![]);
                if let Ok(res) = youtube::fetch_playlist(&self.identifier) {
                    if let Some(playlist) = res.items.iter().nth(0) {
                        self.update_with_yt_playlist(&playlist, &items);
                    }
                }
                self.disable();
            },
            Provider::SoundCloud => {
                match soundcloud::fetch_playlist(&self.identifier) {
                    Ok(playlist) => self.update_with_sc_playlist(&playlist),
                    Err(_)       => self.disable()
                };
            },
            Provider::AppleMusic => {
                let country = apple_music::country(&self.url);
                match apple_music::fetch_playlist(&self.identifier, &country) {
                    Ok(playlist) => self.update_with_am_playlist(&playlist),
                    Err(_)       => self.disable(),
                };
            },
            Provider::Spotify => {
                if let Some(owner_id) = self.clone().owner_id {
                    match spotify::fetch_playlist(&owner_id, &self.identifier) {
                        Ok(playlist) => {
                            self.update_with_sp_playlist(&playlist);
                        },
                        Err(_)       => {
                            self.disable();
                        },
                    }
                }
            },
            _ => (),
        };
        match self.state {
            State::Alive => Ok(()),
            State::Dead  => Err(Error::NotFound),
        }
    }
    fn find_by_entry_id(entry_id: Uuid) -> Vec<Playlist> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(
            &format!("SELECT {} FROM playlists p LEFT JOIN playlist_entries pe
                        ON p.id = pe.playlist_id
                        WHERE pe.entry_id = $1
                        ORDER BY p.published_at DESC",
                     Playlist::props_str("p."))).unwrap();
        let rows = stmt.query(&[&entry_id]).unwrap();
        Playlist::rows_to_items(rows)
    }
}

impl Playlist {
    pub fn find_actives() -> Vec<Playlist> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(
            &format!("SELECT {} FROM playlists WHERE velocity > 0 ORDER BY updated_at ASC",
                     Playlist::props_str(""))).unwrap();
        let rows = stmt.query(&[]).unwrap();
        Playlist::rows_to_items(rows)
    }

    pub fn find_by_tracks(ids: &Vec<Uuid>) -> Result<BTreeMap<Uuid, Vec<Playlist>>, Error> {
        let conn = conn().unwrap();
        let sql = format!("SELECT {}, playlist_tracks.track_id FROM playlists
                      LEFT OUTER JOIN playlist_tracks ON playlist_tracks.playlist_id = playlists.id
                      WHERE playlist_tracks.track_id = ANY($1) ORDER BY playlist_tracks.updated_at DESC LIMIT {}",
                          Playlist::props_str("playlists."), ids.len() * 20);
        let stmt = conn.prepare(&sql).unwrap();
        let rows = stmt.query(&[&ids]).unwrap();
        let mut items: BTreeMap<Uuid, Vec<Playlist>> = BTreeMap::new();
        for id in ids.iter() {
            items.insert(*id, vec![]);
        }
        for row in rows.iter() {
            let id: Uuid = row.get(PROPS.len());
            if let Some(playlists) = items.get_mut(&id) {
                playlists.push(Playlist::row_to_item(row))
            }
        }
        Ok(items)
    }


    pub fn from_yt_playlist(playlist: &youtube::Playlist, items: &Vec<youtube::PlaylistItem>) -> Playlist {
        Playlist::find_or_create(Provider::YouTube, (*playlist).id.to_string())
            .unwrap()
            .update_with_yt_playlist(playlist, items)
            .clone()
    }

    pub fn from_sp_playlist(playlist: &spotify::Playlist) -> Playlist {
        Playlist::find_or_create(Provider::Spotify, (*playlist).id.to_string())
            .unwrap()
            .update_with_sp_playlist(playlist)
            .clone()
    }

    pub fn from_sc_playlist(playlist: &soundcloud::Playlist) -> Playlist {
        Playlist::find_or_create(Provider::SoundCloud, (*playlist).id.to_string())
            .unwrap()
            .update_with_sc_playlist(playlist)
            .clone()
    }

    pub fn from_am_playlist(playlist: &apple_music::Playlist) -> Playlist {
        Playlist::find_or_create(Provider::AppleMusic, (*playlist).id.to_string())
            .unwrap()
            .update_with_am_playlist(playlist)
            .clone()
    }

    fn add_tracks(&mut self, tracks: Vec<Track>) -> Vec<PlaylistTrack> {
        let new_tracks = tracks.iter().map(|t| {
            let mut t = t.clone();
            if let Ok(new_track) = Track::find_or_create(t.provider,
                                                         t.identifier.to_string()) {
                t.id      = new_track.id;
                t.save().and_then(|_| PlaylistTrack::upsert(&self, &t))
            } else {
                Err(Error::Unexpected)
            }
        }).filter(|r| r.is_ok()).map(|r| r.unwrap()).collect::<Vec<_>>();
        self.tracks.append(&mut new_tracks.clone());
        new_tracks
    }

    pub fn update_with_yt_playlist(&mut self, playlist: &youtube::Playlist, items: &Vec<youtube::PlaylistItem>) -> &mut Playlist {
        self.provider      = Provider::YouTube;
        self.identifier    = playlist.id.to_string();
        self.owner_id      = Some(playlist.snippet.channelId.to_string());
        self.owner_name    = Some(playlist.snippet.channelTitle.to_string());
        self.url           = format!("https://www.youtube.com/playlist?list={}", playlist.id);
        self.title         = playlist.snippet.title.to_string();
        self.description   = Some(playlist.snippet.description.to_string());
        self.thumbnail_url = playlist.snippet.get_thumbnail_url();
        self.artwork_url   = playlist.snippet.get_artwork_url();
        self.state         = State::Alive;
        match DateTime::parse_from_rfc3339(&playlist.snippet.publishedAt) {
            Ok(published_at) => self.published_at = published_at.naive_utc(),
            Err(_)           => (),
        }
        let tracks = items.iter()
            .map(|ref i| Track::from_yt_playlist_item(i))
            .collect::<Vec<_>>();
        self.add_tracks(tracks);
        self
    }

    pub fn update_with_sp_playlist(&mut self, playlist: &spotify::Playlist) -> &mut Playlist {
        self.provider       = Provider::Spotify;
        self.identifier     = playlist.id.to_string();
        self.owner_id       = Some(playlist.owner.id.to_string());
        self.owner_name     = playlist.owner.display_name.clone();
        self.url            = playlist.uri.clone();
        self.title          = playlist.name.clone();
        self.description    = playlist.description.clone();
        self.state          = State::Alive;
        if playlist.images.len() > 0 {
            self.artwork_url   = Some(playlist.images[0].url.clone());
            self.thumbnail_url = Some(playlist.images[0].url.clone());
        }
        if playlist.images.len() > 1 {
            self.thumbnail_url = Some(playlist.images[1].url.clone());
        }
        let track_ids = playlist.tracks.items.iter()
            .filter(|ref i| i.track.is_some())
            .map(|ref i| i.track.clone().unwrap())
            .filter(|ref track| track.id.is_some())
            .map(|track| track.id.unwrap().to_string())
            .collect::<Vec<_>>();
        let sp_tracks = spotify::fetch_tracks(track_ids).unwrap_or(vec![]);
        let tracks = sp_tracks.iter()
            .map(|ref t| Track::from_sp_track(t))
            .filter(|ref r| r.is_ok())
            .map(|r| r.unwrap())
            .collect::<Vec<_>>();
        self.add_tracks(tracks);
        self
    }

    pub fn update_with_sc_playlist(&mut self, playlist: &soundcloud::Playlist) -> &mut Playlist {
        self.provider      = Provider::SoundCloud;
        self.identifier    = playlist.id.to_string();
        self.owner_id      = Some(playlist.user.id.to_string());
        self.owner_name    = Some(playlist.user.username.clone());
        self.url           = playlist.permalink_url.to_string();
        self.title         = playlist.title.to_string();
        self.description   = None;
        self.thumbnail_url = playlist.artwork_url.clone();
        self.artwork_url   = playlist.artwork_url.clone();
        self.state         = State::Alive;
        match DateTime::parse_from_str(&playlist.created_at, "%Y/%m/%d %H:%M:%S %z") {
            Ok(published_at) => self.published_at = published_at.naive_utc(),
            Err(_)           => (),
        };
        let tracks = playlist.tracks.iter()
            .map(|ref t| Track::from_sc_track(t))
            .collect::<Vec<_>>();
        self.add_tracks(tracks);
        self
    }

    pub fn update_with_am_playlist(&mut self, playlist: &apple_music::Playlist) -> &mut Playlist {
        let curator = playlist.clone().relationships.map(|r| {
            r.curator.data.clone()
        }).unwrap_or(vec![]);
        self.provider      = Provider::AppleMusic;
        self.identifier    = playlist.id.clone();
        self.owner_id      = curator.first().map(|c| c.id.clone());
        self.owner_name    = playlist.attributes.curator_name.clone();
        self.url           = playlist.attributes.url.clone();
        self.title         = playlist.attributes.name.clone();
        self.description   = playlist.attributes.description.clone().and_then(|d| d.short.clone());
        self.thumbnail_url = playlist.attributes.artwork.clone().map(|a| a.get_thumbnail_url());
        self.artwork_url   = playlist.attributes.artwork.clone().map(|a| a.get_artwork_url());
        self.state         = State::Alive;
        self
    }

    pub fn disable(&mut self) -> &mut Playlist {
        self.state = State::Dead;
        self
    }

    pub fn delete(&self) -> Result<(), Error> {
        let conn = conn()?;
        let stmt = conn.prepare("DELETE FROM playlists WHERE id=$1")?;
        let result = stmt.query(&[&self.id]);
        match result {
            Ok(_)  => Ok(()),
            Err(_) => Err(Error::Unexpected)
        }
    }

    pub fn fetch_tracks(&mut self) -> Result<Vec<PlaylistTrack>, Error> {
        match self.provider {
            Provider::YouTube    => Ok(vec![]),
            Provider::SoundCloud => Ok(vec![]),
            Provider::AppleMusic => self.fetch_apple_music_tracks(),
            Provider::Spotify    => self.fetch_spotify_tracks(),
            _                    => Ok(vec![]),
        }
    }

    pub fn fetch_apple_music_tracks(&mut self) -> Result<Vec<PlaylistTrack>, Error> {
        let country = apple_music::country(&self.url);
        let playlist = apple_music::fetch_playlist(&country, &self.identifier)?;
        let songs = playlist.get_songs();
        let song_ids = songs.iter().map(|song| song.id.clone()).collect::<Vec<String>>();
        let songs = apple_music::fetch_songs(&country, song_ids).unwrap_or(vec![]);
        Ok(self.add_tracks(songs.iter().map(|song| Track::from_am_song(song)).collect()))
    }

    pub fn fetch_spotify_tracks(&mut self) -> Result<Vec<PlaylistTrack>, Error> {
        let mut items = vec![];
        let owner_id = self.clone().owner_id.ok_or(Error::Unexpected)?;

        let mut page = spotify::fetch_playlist_tracks(&owner_id, &self.identifier)?;
        items.append(&mut self.add_tracks(
            page.items.iter()
                .filter(|pt| pt.track.is_some())
                .map(|pt| Track::from_sp_track(&pt.track.clone().unwrap()))
                .filter(|ref r| r.is_ok())
                .map(|r| r.unwrap())
                .collect())
        );
        while page.next.is_some() {
            page = page.fetch_next()?;
            items.append(&mut self.add_tracks(
                page.items.iter()
                    .filter(|pt| pt.track.is_some())
                    .map(|pt| Track::from_sp_track(&pt.track.clone().unwrap()))
                    .filter(|ref r| r.is_ok())
                    .map(|r| r.unwrap())
                    .collect())
            );
        }
        Ok(items)
    }
}

#[cfg(test)]
mod test {
    use model::enclosure::Enclosure;
    use model::Model;
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
