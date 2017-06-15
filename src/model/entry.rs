use postgres;
use uuid::Uuid;
use error::Error;
use chrono::{NaiveDateTime, UTC};
use super::{conn, Model};
use model::enclosure::Enclosure;
use scraper;
use Track;
use Playlist;
use Album;
use model::PaginatedCollection;
use serde_json::Value;
use feed_rs;

static PROPS: [&'static str; 20]  = ["id",
                                     "url",
                                     "title",
                                     "description",
                                     "visual_url",
                                     "locale",

                                     "summary",
                                     "content",
                                     "author",
                                     "crawled",
                                     "published",
                                     "updated",
                                     "fingerprint",
                                     "origin_id",
                                     "alternate",
                                     "keywords",
                                     "enclosure",

                                     "feed_id",

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
    pub summary:     Option<String>,
    pub content:     Option<String>,
    pub author:      Option<String>,
    pub crawled:     NaiveDateTime,
    pub published:   NaiveDateTime,
    pub updated:     Option<NaiveDateTime>,
    pub fingerprint: String,
    pub origin_id:   String,
    pub alternate:   Value,
    pub keywords:    Value,
    pub enclosure:   Value,
    pub feed_id:     Option<Uuid>,
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
                summary:     row.get(6),
                content:     row.get(7),
                author:      row.get(8),
                crawled:     row.get(9),
                published:   row.get(10),
                updated:     row.get(11),
                fingerprint: row.get(12),
                origin_id:   row.get(13),
                alternate:   row.get(14),
                keywords:    row.get(15),
                enclosure:   row.get(16),
                feed_id:     row.get(17),
                created_at:  row.get(18),
                updated_at:  row.get(19),
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
                                   summary     = $7,
                                   content     = $8,
                                   author      = $9,
                                   crawled     = $10,
                                   published   = $11,
                                   updated     = $12,
                                   fingerprint = $13,
                                   origin_id   = $14,
                                   alternate   = $15,
                                   keywords    = $16,
                                   enclosure   = $17,
                                   feed_id     = $18,
                                   created_at  = $19,
                                   updated_at  = $20
                                 WHERE id = $1"));
        let result = stmt.query(&[&self.id,
                                  &self.url,
                                  &self.title,
                                  &self.description,
                                  &self.visual_url,
                                  &self.locale,
                                  &self.summary,
                                  &self.content,
                                  &self.author,
                                  &self.crawled,
                                  &self.published,
                                  &self.updated,
                                  &self.fingerprint,
                                  &self.origin_id,
                                  &self.alternate,
                                  &self.keywords,
                                  &self.enclosure,
                                  &self.feed_id,
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

                summary:     None,
                content:     None,
                author:      None,
                crawled:     UTC::now().naive_utc(),
                published:   UTC::now().naive_utc(),
                updated:     None,
                fingerprint: "".to_string(),
                origin_id:   "".to_string(),
                alternate:   Value::Null,
                keywords:    Value::Null,
                enclosure:   Value::Null,
                feed_id:     None,

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
            Err(Error::NotFound) => Entry::create_by_url(url),
            Err(e) => Err(e),
        }
    }

    pub fn find_by_feed_id(feed_id: Uuid, newer_than: Option<NaiveDateTime>, page: i64, per_page: i64) -> PaginatedCollection<Entry> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(
            &format!("SELECT {} FROM entries
                        WHERE entries.feed_id = $1 AND entries.published >= $2
                        ORDER BY entries.published DESC
                        LIMIT $4 OFFSET $3",
                     Entry::props_str(""))).unwrap();
        let offset = page * per_page;
        let published = newer_than.unwrap_or(NaiveDateTime::from_timestamp(0, 0));
        let rows   = stmt.query(&[&feed_id, &published, &offset, &per_page]).unwrap();
        let items  = Self::rows_to_items(rows);
        let mut total: i64 = 0;
        let sql = "SELECT COUNT(*) FROM entries WHERE entries.feed_id = $1";
        for row in conn.query(&sql, &[&feed_id]).unwrap().iter() {
            total = row.get(0);
        }
        PaginatedCollection {
            page:     page,
            per_page: per_page,
            total:    total,
            items:    items,
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

                summary:     None,
                content:     None,
                author:      None,
                crawled:     UTC::now().naive_utc(),
                published:   UTC::now().naive_utc(),
                updated:     None,
                fingerprint: "".to_string(),
                origin_id:   "".to_string(),
                alternate:   Value::Null,
                keywords:    Value::Null,
                enclosure:   Value::Null,
                feed_id:     None,

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

    pub fn update_with_feed_entry(&mut self, entry: &feed_rs::Entry) {
        self.title       = entry.title.clone();
        self.summary     = entry.summary.clone();
        self.content     = entry.content.clone();
        self.author      = entry.author.clone();
        self.crawled     = UTC::now().naive_utc();
        self.published   = entry.published;
        self.updated     = entry.updated;
        self.fingerprint = entry.fingerprint.clone();
        self.origin_id   = entry.id.clone();
        self.alternate   = json!(entry.alternate);
        self.keywords    = json!(entry.keywords);
        self.enclosure   = json!(entry.enclosure);

        self.updated_at  = UTC::now().naive_utc();
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
