use postgres;
use uuid::Uuid;
use std::fmt;
use std::collections::BTreeMap;
use chrono::{NaiveDateTime, Utc, DateTime};

use apple_music;
use youtube;
use youtube::HasThumbnail;
use soundcloud;
use spotify;
use lemoned;
use error::Error;
use super::{conn, Model};
use model::enclosure::Enclosure;
use model::provider::Provider;
use model::state::State;
use model::artist::Artist;
use model::album::Album;
use model::playlist::Playlist;

pub static PROPS: [&'static str; 16]  = ["id",
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub album:         Option<Album>,
    pub artists:       Option<Vec<Artist>>,
    pub playlists:     Option<Vec<Playlist>>,
}

impl PartialEq for Track {
    fn eq(&self, t: &Track) -> bool {
        return self.identifier == t.identifier && self.provider == t.provider
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.provider, self.identifier)
    }
}

impl<'a> Model<'a> for Track {
    fn table_name() -> String {
        "tracks".to_string()
    }
    fn props_str(prefix: &str) -> String {
        PROPS
            .iter()
            .map(|&p| format!("{}{}", prefix, p))
            .collect::<Vec<String>>().join(",")
    }
    fn row_to_item(row: postgres::rows::Row) -> Track {
        Track {
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
            album:         None,
            artists:       None,
            playlists:     None,
        }
    }
    fn create(&self) -> Result<Track, Error> {
        let conn = conn()?;
        let stmt = conn.prepare("INSERT INTO tracks (provider, identifier, url, title)
                                      VALUES ($1, $2, $3, $4) RETURNING id")?;
        let rows = stmt.query(&[&self.provider.to_string(), &self.identifier, &self.url, &self.title])?;
        let mut track = self.clone();
        for row in rows.iter() {
            track.id = row.get(0);
        }
        Ok(track)
    }

    fn save(&mut self) -> Result<(), Error> {
        self.updated_at = Utc::now().naive_utc();
        let conn = conn()?;
        let stmt = conn.prepare("UPDATE tracks SET
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

    fn set_relations(tracks: &mut Vec<Track>) -> Result<(), Error> {
        let ids = tracks.iter().map(|i| i.id).collect();
        let artists_map   = Artist::find_by_tracks(&ids)?;
        let playlists_map = Playlist::find_by_tracks(&ids)?;
        for track in tracks {
            if let Some(ref mut artists) = artists_map.get(&track.id) {
                track.artists = Some(artists.clone())
            }
            if let Some(ref mut playlists) = playlists_map.get(&track.id) {
                track.playlists = Some(playlists.clone())
            }
        }
        Ok(())
    }
}

impl<'a> Enclosure<'a> for Track {
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
            published_at:  Utc::now().naive_utc(),
            created_at:    Utc::now().naive_utc(),
            updated_at:    Utc::now().naive_utc(),
            state:         State::Alive,
            album:         None,
            artists:       None,
            playlists:     None,
        }
    }

    fn set_url(&mut self, url: String) -> &mut Track {
        self.url = url;
        self
    }

    fn set_owner_id(&mut self, owner_id: Option<String>) -> &mut Track {
        self.owner_id = owner_id;
        self
    }

    fn fetch_props(&mut self) -> Result<(), Error> {
        match self.provider {
            Provider::YouTube => match youtube::fetch_video(&self.identifier) {
                Ok(video) => self.update_with_yt_video(&video),
                Err(_)    => self.disable(),
            },
            Provider::SoundCloud => match soundcloud::fetch_track(&self.identifier) {
                Ok(sc_track) => self.update_with_sc_track(&sc_track),
                Err(_)       => self.disable(),
            },
            Provider::AppleMusic => {
                let country = apple_music::country(&self.url);
                match apple_music::fetch_song(&self.identifier, &country) {
                    Ok(song) => self.update_with_am_song(&song),
                    Err(_)   => self.disable(),
                }
            },
            Provider::Spotify => match spotify::fetch_track(&self.identifier) {
                Ok(sp_track) => self.update_with_sp_track(&sp_track),
                Err(_)       => self.disable(),
            },
            Provider::Custom => match lemoned::fetch_track(&self.identifier) {
                Ok(le_track) => self.update_with_le_track(&le_track),
                Err(_)       => self.disable(),
            },
            _ => self,
        };
        match self.state {
            State::Alive => Ok(()),
            State::Dead  => Err(Error::NotFound),
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
        Track::find_or_create(Provider::AppleMusic, identifier.to_string())
            .unwrap()
            .update_with_am_song(song)
            .clone()
    }
    pub fn from_yt_playlist_item(item: &youtube::PlaylistItem) -> Track {
        let identifier = (*item).snippet.resourceId["videoId"].to_string();
        Track::find_or_create(Provider::YouTube, identifier.to_string())
            .unwrap()
            .update_with_yt_playlist_item(item)
            .clone()
    }
    pub fn from_yt_video(video: &youtube::Video) -> Track {
        Track::find_or_create(Provider::YouTube, (*video).id.to_string())
            .unwrap()
            .update_with_yt_video(video)
            .clone()
    }
    pub fn from_sc_track(track: &soundcloud::Track) -> Track {
        Track::find_or_create(Provider::SoundCloud, (*track).id.to_string())
            .unwrap()
            .update_with_sc_track(track)
            .clone()
    }
    pub fn from_sp_track(track: &spotify::Track) -> Result<Track, Error> {
        let track_id = track.clone().id.ok_or(Error::Unexpected)?;
        Ok(Track::find_or_create(Provider::Spotify, track_id.to_string())
            .unwrap()
            .update_with_sp_track(track)
            .clone())
    }
    fn add_artist(&mut self, artist: &Artist) -> Result<(), Error> {
        let conn = conn()?;
        let stmt = conn.prepare("INSERT INTO track_artists (track_id, artist_id) VALUES ($1, $2)")?;
        stmt.query(&[&self.id, &artist.id])?;
        match self.artists {
            Some(ref mut artists) => artists.push(artist.clone()),
            None                  => self.artists = Some(vec![artist.clone()]),

        }
        Ok(())
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
    pub fn find_by_artist(artist_id: Uuid) -> Vec<Track> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(
            &format!("SELECT {} FROM tracks
                      LEFT OUTER JOIN track_artists ON track_artists.track_id = tracks.id
                      WHERE track_artists.artist_id = $1 ORDER BY tracks.created_at DESC",
                     Track::props_str("tracks."))).unwrap();
        let rows = stmt.query(&[&artist_id]).unwrap();
        Track::rows_to_items(rows)
    }
    pub fn find_by_album(album_id: Uuid) -> Vec<Track> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(
            &format!("SELECT {} FROM tracks
                      LEFT OUTER JOIN album_tracks ON album_tracks.track_id = tracks.id
                      WHERE album_tracks.album_id = $1",
                     Track::props_str("tracks."))).unwrap();
        let rows = stmt.query(&[&album_id]).unwrap();
        Track::rows_to_items(rows)
    }
    pub fn find_by_albums(album_ids: &Vec<Uuid>) -> Result<BTreeMap<Uuid, Vec<Track>>, Error> {
        let conn = conn().unwrap();
        let sql = format!("SELECT {}, album_tracks.album_id FROM tracks
                      LEFT OUTER JOIN album_tracks ON album_tracks.track_id = tracks.id
                      WHERE album_tracks.album_id = ANY($1)", Track::props_str("tracks."));
        let stmt = conn.prepare(&sql).unwrap();
        let rows = stmt.query(&[&album_ids]).unwrap();
        let mut items: BTreeMap<Uuid, Vec<Track>> = BTreeMap::new();
        for id in album_ids.iter() {
            items.insert(*id, vec![]);
        }
        for row in rows.iter() {
            let id: Uuid = row.get(PROPS.len());
            if let Some(tracks) = items.get_mut(&id) {
                tracks.push(Self::row_to_item(row))
            }
        }
        Ok(items)
    }
    pub fn find_by_provider(provider: &Provider) -> Vec<Track> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(
            &format!("SELECT {} FROM tracks WHERE tracks.provider = $1
                        ORDER BY tracks.created_at DESC",
                     Track::props_str(""))).unwrap();
        let rows = stmt.query(&[&(*provider).to_string()]).unwrap();
        Track::rows_to_items(rows)
    }
    pub fn update_with_am_song(&mut self, song: &apple_music::Song) -> &mut Track {
        let song_artists = song.clone().relationships.map(|r| {
            r.artists.data.clone()
        });
        self.provider      = Provider::AppleMusic;
        self.identifier    = song.id.to_string();
        self.url           = song.attributes.url.to_string();
        self.title         = song.attributes.name.to_string();
        self.description   = None;
        self.thumbnail_url = Some(song.attributes.artwork.get_thumbnail_url());
        self.artwork_url   = Some(song.attributes.artwork.get_artwork_url());
        self.audio_url     = song.attributes.previews.first().map(|p| {
            p.url.clone()
        });
        self.state         = State::Alive;
        if let Some(song_artist) = song_artists.clone().and_then(|a| a.first().map(|a| a.clone())) {
            let artist_name    = song_artist.attributes.name.clone();
            self.owner_id      = Some(song_artist.id.to_string());
            self.owner_name    = Some(artist_name.to_string());
        }

        let country = apple_music::country(&self.url);
        if let Some(song_artists) = song_artists.clone() {
            let artist_ids = song_artists.iter().map(|a| a.id.clone()).collect::<Vec<String>>();
            let artists = apple_music::fetch_artists(&country, artist_ids).unwrap_or(vec![]);
            self.add_artists(artists.iter().map(|a| Artist::from_am_artist(a)).collect());
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
        if let Ok(channel) = youtube::fetch_channel(&s.channelId) {
            self.add_artists(vec![Artist::from_yt_channel(&channel)]);
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
        if let Ok(channel) = youtube::fetch_channel(&s.channelId) {
            self.add_artists(vec![Artist::from_yt_channel(&channel)]);
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
        self.add_artists(vec![Artist::from_sc_user(&track.user)]);
        self
    }

    pub fn update_with_sp_track(&mut self, track: &spotify::Track) -> &mut Track {
        if track.id == None {
            return self
        }
        self.provider       = Provider::Spotify;
        self.identifier     = track.clone().id.unwrap();
        if track.artists.len() > 0 {
            self.owner_id   = Some(track.artists[0].id.clone());
            self.owner_name = Some(track.artists[0].name.clone());
        }
        self.url            = track.uri.clone();
        self.title          = track.name.clone();
        self.description    = None;
        self.audio_url      = track.preview_url.clone();
        self.state          = State::Alive;
        self.published_at   = Utc::now().naive_utc();
        if let Some(album) = track.album.clone() {
            self.update_with_sp_album(&album);
        }

        let artist_ids = track.artists.iter().map(|a| a.id.clone()).collect::<Vec<String>>();
        let sp_artists = spotify::fetch_artists(artist_ids).unwrap_or_default();
        let artists = sp_artists.iter().map(|ref a| Artist::from_sp_artist(a))
            .collect::<Vec<_>>();
        self.add_artists(artists);

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

    pub fn update_with_le_track(&mut self, track: &lemoned::Track) -> &mut Track {
        self.provider      = Provider::Custom;
        self.url           = track.url.clone();
        self.title         = track.title.clone().unwrap_or("".to_string());
        self.description   = track.description.clone();
        self.thumbnail_url = track.thumbnail_url.clone();
        self.artwork_url   = track.artwork_url.clone();
        self.audio_url     = track.audio_url.clone();
        self.published_at  = NaiveDateTime::from_timestamp(track.published_at.timestamp(), 0);
        self.owner_id      = track.clone().artist.map(|a| a.id.to_string());
        self.owner_name    = track.clone().artist.map(|a| a.name);
        self
    }

    pub fn disable(&mut self) -> &mut Track {
        self.state = State::Dead;
        self
    }
}
