use postgres;
use uuid::Uuid;
use error::Error;
use chrono::{NaiveDateTime, Utc};
use super::{conn, Model};
use model::enclosure::Enclosure;
use scraper;
use Track;
use Playlist;
use Album;
use model::PaginatedCollection;
use serde_json::Value;
use feed_rs;

static PROPS: [&'static str; 21]  = ["id",
                                     "url",
                                     "title",
                                     "description",
                                     "visual_url",
                                     "locale",

                                     "summary",
                                     "content",
                                     "text",
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
    pub text:        Option<String>,
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
                text:        row.get(8),
                author:      row.get(9),
                crawled:     row.get(10),
                published:   row.get(11),
                updated:     row.get(12),
                fingerprint: row.get(13),
                origin_id:   row.get(14),
                alternate:   row.get(15),
                keywords:    row.get(16),
                enclosure:   row.get(17),
                feed_id:     row.get(18),
                created_at:  row.get(19),
                updated_at:  row.get(20),
                tracks:      Track::find_by_entry_id(row.get(0)),
                playlists:   Playlist::find_by_entry_id(row.get(0)),
                albums:      Album::find_by_entry_id(row.get(0)),
            })
        }
        entries
    }
    fn create(&self) -> Result<Entry, Error> {
        let conn = try!(conn());
        let stmt = try!(conn.prepare("INSERT INTO entries (url, published) VALUES ($1, $2) RETURNING id"));
        let rows = try!(stmt.query(&[&self.url, &NaiveDateTime::from_timestamp(0, 0)]));
        let mut entry = self.clone();
        for row in rows.iter() {
            entry.id = row.get(0);
        }
        Ok(entry)
    }
    fn save(&mut self) -> Result<(), Error> {
        self.updated_at = Utc::now().naive_utc();
        let conn = try!(conn());
        let stmt = try!(conn.prepare("UPDATE entries SET
                                   url         = $2,
                                   title       = $3,
                                   description = $4,
                                   visual_url  = $5,
                                   locale      = $6,
                                   summary     = $7,
                                   content     = $8,
                                   text        = $9,
                                   author      = $10,
                                   crawled     = $11,
                                   published   = $12,
                                   updated     = $13,
                                   fingerprint = $14,
                                   origin_id   = $15,
                                   alternate   = $16,
                                   keywords    = $17,
                                   enclosure   = $18,
                                   feed_id     = $19,
                                   created_at  = $20,
                                   updated_at  = $21
                                 WHERE id = $1"));
        let result = stmt.query(&[&self.id,
                                  &self.url,
                                  &self.title,
                                  &self.description,
                                  &self.visual_url,
                                  &self.locale,
                                  &self.summary,
                                  &self.content,
                                  &self.text,
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
                text:        None,
                author:      None,
                crawled:     NaiveDateTime::from_timestamp(0, 0),
                published:   NaiveDateTime::from_timestamp(0, 0), // exclude from api response
                updated:     None,
                fingerprint: "".to_string(),
                origin_id:   "".to_string(),
                alternate:   Value::Null,
                keywords:    Value::Null,
                enclosure:   Value::Null,
                feed_id:     None,

                created_at:  Utc::now().naive_utc(),
                updated_at:  Utc::now().naive_utc(),

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
        let published = newer_than.unwrap_or(NaiveDateTime::from_timestamp(1000, 0)); // ignore 0 timestamp
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
        let stmt = try!(conn.prepare("INSERT INTO entries (url, published) VALUES ($1, $2) RETURNING id"));
        let rows = try!(stmt.query(&[&url, &NaiveDateTime::from_timestamp(0, 0)]));
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
                text:        None,
                author:      None,
                crawled:     Utc::now().naive_utc(),
                published:   Utc::now().naive_utc(),
                updated:     None,
                fingerprint: "".to_string(),
                origin_id:   "".to_string(),
                alternate:   Value::Null,
                keywords:    Value::Null,
                enclosure:   Value::Null,
                feed_id:     None,

                created_at:  Utc::now().naive_utc(),
                updated_at:  Utc::now().naive_utc(),

                tracks:      Vec::new(),
                playlists:   Vec::new(),
                albums:      Vec::new(),
            };
            return Ok(entry);
        }
        Err(Error::Unexpected)
    }

    pub fn is_valid(&self) -> bool {
        self.published.timestamp() >= 1000 && self.feed_id != None
    }

    pub fn find_or_create_by_url_if_invalid(url: String) -> Result<Entry, Error> {
        match Entry::find_by_url(&url) {
            Ok(entry) => {
                if !entry.is_valid() {
                    Ok(entry)
                } else {
                    Err(Error::NotFound)
                }
            },
            Err(_) => {
                Entry::create_by_url(url)
            }
        }
    }

    pub fn update_with_feed_entry(&mut self, entry: &feed_rs::Entry) {
        self.title       = entry.title.clone().map(|s| s.trim().to_string());
        self.summary     = entry.summary.clone().map(|s| s.trim().to_string());
        self.content     = entry.content.clone().map(|s| s.trim().to_string());
        self.author      = entry.author.clone();
        self.crawled     = Utc::now().naive_utc();
        self.published   = entry.published;
        self.updated     = entry.updated;
        self.fingerprint = entry.fingerprint.clone();
        self.origin_id   = entry.id.clone();
        self.alternate   = json!(entry.alternate);
        self.keywords    = json!(entry.keywords);
        self.enclosure   = json!(entry.enclosure);

        self.updated_at  = Utc::now().naive_utc();
    }

    pub fn has_title(&self) -> bool {
        let title = &self.title.clone().unwrap_or("".to_string());
        !title.is_empty()
    }

    pub fn playlistify(&mut self) -> Result<(), Error> {
        let product = try!(scraper::extract(&self.url));
        match product.og_obj {
            Some(og_obj) => {
                if !self.has_title() {
                    self.title = Some(og_obj.title);
                }
                self.description = og_obj.description;
                self.locale      = og_obj.locale;
                self.visual_url  = og_obj.images.first().map(|i| i.url.clone());
            },
            None => (),
        }
        for t in product.tracks {
            let new_track = try!(Track::find_or_create(t.provider, t.identifier.to_string()));
            let mut track = t.clone();
            track.id      = new_track.id;
            try!(track.fetch_detail().save());
            match self.tracks.iter().find(|&t| t.id == track.id) {
                Some(_) => (),
                None    => try!(self.add_track(track.clone())),
            }
        }
        for p in product.playlists {
            let new_playlist = try!(Playlist::find_or_create(p.provider, p.identifier.to_string()));
            let mut playlist = p.clone();
            playlist.id      = new_playlist.id;
            try!(playlist.save());
            match self.playlists.iter().find(|&p| p.id == playlist.id) {
                Some(_) => (),
                None    => try!(self.add_playlist(playlist.clone())),
            }
        }
        for a in product.albums {
            let new_album = try!(Album::find_or_create(a.provider, a.identifier.to_string()));
            let mut album = a.clone();
            album.id      = new_album.id;
            try!(album.save());
            match self.albums.iter().find(|&a| a.id == album.id) {
                Some(_) => (),
                None    => try!(self.add_album(album.clone())),
            }
        }

        Ok(())
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
