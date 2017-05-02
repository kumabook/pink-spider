use postgres;
use uuid::Uuid;
use error::Error;
use chrono::{NaiveDateTime, UTC};
use super::{conn, Model};
use model::enclosure::Enclosure;
use Track;
use Playlist;
use Album;

static PROPS: [&'static str; 8]  = ["id",
                                    "url",
                                    "title",
                                    "description",
                                    "visual_url",
                                    "locale",
                                    "created_at",
                                    "updated_at"];

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    pub id:          Uuid,
    pub url:         String,
    pub title:       Option<String>,
    pub description: Option<String>,
    pub visual_url:  Option<String>,
    pub locale:      Option<String>,
    pub created_at:  NaiveDateTime,
    pub updated_at:  NaiveDateTime,
    pub tracks:      Vec<Track>,
    pub playlists:   Vec<Playlist>,
    pub albums:      Vec<Album>,
}

impl<'a> Model<'a> for Entry {
    fn table_name() -> String {
        "entries".to_string()
    }
    fn props_str(prefix: &str) -> String {
        PROPS
            .iter()
            .map(|&p| format!("{}{}", prefix, p))
            .collect::<Vec<String>>().join(",")
    }
    fn rows_to_items(rows: postgres::rows::Rows) -> Vec<Entry> {
        let mut entries = Vec::new();
        for row in rows.iter() {
            entries.push(Entry {
                id:          row.get(0),
                url:         row.get(1),
                title:       row.get(2),
                description: row.get(3),
                visual_url:  row.get(4),
                locale:      row.get(5),
                created_at:  row.get(6),
                updated_at:  row.get(7),
                tracks:      Track::find_by_entry_id(row.get(0)),
                playlists:   Playlist::find_by_entry_id(row.get(0)),
                albums:      Album::find_by_entry_id(row.get(0)),
            })
        }
        entries
    }
    fn create(&self) -> Result<Entry, Error> {
        let conn = try!(conn());
        let stmt = try!(conn.prepare("INSERT INTO entries (url) VALUES ($1) RETURNING id"));
        let rows = try!(stmt.query(&[&self.url]));
        let mut entry = self.clone();
        for row in rows.iter() {
            entry.id = row.get(0);
        }
        Ok(entry)
    }
    fn save(&mut self) -> Result<(), Error> {
        self.updated_at = UTC::now().naive_utc();
        let conn = try!(conn());
        let stmt = try!(conn.prepare("UPDATE entries SET
                                   url         = $2,
                                   title       = $3,
                                   description = $4,
                                   visual_url  = $5,
                                   locale      = $6,
                                   created_at  = $7,
                                   updated_at  = $8
                                 WHERE id = $1"));
        let result = stmt.query(&[&self.id,
                                  &self.url,
                                  &self.title,
                                  &self.description,
                                  &self.visual_url,
                                  &self.locale,
                                  &self.created_at,
                                  &self.updated_at]);
        try!(result);
        Ok(())
    }
}

impl Entry {
    pub fn new(url: String) -> Result<Entry, Error> {
        let conn = try!(conn());
        let stmt = try!(conn.prepare("INSERT INTO entries (url) VALUES ($1) RETURNING id"));
        let rows = try!(stmt.query(&[&url]));
        for row in rows.iter() {
            let entry = Entry {
                id:          row.get(0),
                url:         url,
                title:       None,
                description: None,
                visual_url:  None,
                locale:      None,
                created_at:  UTC::now().naive_utc(),
                updated_at:  UTC::now().naive_utc(),
                tracks:      Vec::new(),
                playlists:   Vec::new(),
                albums:      Vec::new(),
            };
            return Ok(entry);
        }
        Err(Error::Unexpected)
    }

    pub fn find_by_url(url: &str) -> Result<Entry, Error> {
        let conn = try!(conn());
        let stmt = try!(conn.prepare(
            &format!("SELECT {} FROM entries
                        WHERE url = $1", Self::props_str(""))));
        let rows = try!(stmt.query(&[&url]));
        let entries = Entry::rows_to_items(rows);
        if entries.len() > 0 {
            return Ok(entries[0].clone());
        }
        return Err(Error::NotFound)
    }

    pub fn find_or_create_by_url(url: String) -> Result<Entry, Error> {
        match Entry::find_by_url(&url) {
            Ok(entry) => Ok(entry),
            Err(_)    => Entry::create_by_url(url)
        }
    }

    pub fn create_by_url(url: String) -> Result<Entry, Error> {
        let conn = try!(conn());
        let stmt = try!(conn.prepare("INSERT INTO entries (url) VALUES ($1) RETURNING id"));
        let rows = try!(stmt.query(&[&url]));
        for row in rows.iter() {
            let entry = Entry {
                id:          row.get(0),
                url:         url,
                title:       None,
                description: None,
                visual_url:  None,
                locale:      None,
                created_at:  UTC::now().naive_utc(),
                updated_at:  UTC::now().naive_utc(),
                tracks:      Vec::new(),
                playlists:   Vec::new(),
                albums:      Vec::new(),
            };
            return Ok(entry);
        }
        Err(Error::Unexpected)
    }

    pub fn add_track(&mut self, track: Track) -> Result<(), Error> {
        let conn = try!(conn());
        let stmt = try!(conn.prepare("INSERT INTO track_entries (track_id, entry_id)
                                 VALUES ($1, $2)"));
        try!(stmt.query(&[&track.id, &self.id]));
        self.tracks.push(track);
        Ok(())
    }

    pub fn add_playlist(&mut self, playlist: Playlist) -> Result<(), Error> {
        let conn = try!(conn());
        let stmt = try!(conn.prepare("INSERT INTO playlist_entries (playlist_id, entry_id)
                                 VALUES ($1, $2)"));
        try!(stmt.query(&[&playlist.id, &self.id]));
        self.playlists.push(playlist);
        Ok(())
    }

    pub fn add_album(&mut self, album: Album) -> Result<(), Error> {
        let conn = try!(conn());
        let stmt = try!(conn.prepare("INSERT INTO album_entries (album_id, entry_id)
                                 VALUES ($1, $2)"));
        try!(stmt.query(&[&album.id, &self.id]));
        self.albums.push(album);
        Ok(())
    }
}
