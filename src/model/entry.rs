use std::collections::BTreeMap;
use rustc_serialize::json::{ToJson, Json};
use postgres;
use uuid::Uuid;
use error::Error;
use super::{conn, PaginatedCollection};
use Track;

static PROPS: [&'static str; 6]  = ["id",
                                    "url",
                                    "title",
                                    "description",
                                    "visual_url",
                                    "locale"];

fn props_str() -> String {
    PROPS.join(",")
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub id:          Uuid,
    pub url:         String,
    pub title:       Option<String>,
    pub description: Option<String>,
    pub visual_url:  Option<String>,
    pub locale:      Option<String>,
    pub tracks:      Vec<Track>,
}

impl ToJson for Entry {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        let tracks = Json::Array(self.tracks.iter().map(|x| x.to_json()).collect());
        d.insert("id".to_string()         , self.id.to_string().to_json());
        d.insert("url".to_string()        , self.url.to_json());
        d.insert("title".to_string()      , self.title.to_json());
        d.insert("description".to_string(), self.description.to_json());
        d.insert("visual_url".to_string() , self.visual_url.to_json());
        d.insert("locale".to_string()     , self.locale.to_json());
        d.insert("tracks".to_string()     , tracks);
        Json::Object(d)
    }
}

impl Entry {
    fn rows_to_entries(rows: postgres::rows::Rows) -> Vec<Entry> {
        let mut entries = Vec::new();
        for row in rows.iter() {
            entries.push(Entry {
                    id: row.get(0),
                   url: row.get(1),
                 title: row.get(2),
           description: row.get(3),
            visual_url: row.get(4),
                locale: row.get(5),
                tracks: Track::find_by_entry_id(row.get(0)),
            })
        }
        entries
    }

    pub fn find_by_id(id: String) -> Result<Entry, Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(
            &format!("SELECT {} FROM entries
                        WHERE id = $1", props_str())).unwrap();
        let rows = stmt.query(&[&id]).unwrap();
        let entries = Entry::rows_to_entries(rows);
        if entries.len() > 0 {
            return Ok(entries[0].clone());
        }
        return Err(Error::NotFound)
    }

    pub fn find_by_url(url: &str) -> Result<Entry, Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(
            &format!("SELECT {} FROM entries
                        WHERE url = $1", props_str())).unwrap();
        let rows = stmt.query(&[&url]).unwrap();
        let entries = Entry::rows_to_entries(rows);
        if entries.len() > 0 {
            return Ok(entries[0].clone());
        }
        return Err(Error::NotFound)
    }

    pub fn find(page: i64, per_page: i64) -> PaginatedCollection<Entry> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(
            &format!("SELECT {} FROM entries LIMIT $2 OFFSET $1", props_str())).unwrap();
        let offset = page * per_page;
        let rows = stmt.query(&[&offset, &per_page]).unwrap();
        let entries = Entry::rows_to_entries(rows);
        let mut total: i64 = 0;
        for row in conn.query("SELECT COUNT(*) FROM entries", &[]).unwrap().iter() {
            total = row.get(0);
        }
        PaginatedCollection {
            page:     page,
            per_page: per_page,
            total:    total,
            items:    entries,
        }
    }

    pub fn find_or_create_by_url(url: String) -> Result<Entry, Error> {
        match Entry::find_by_url(&url) {
            Ok(entry) => Ok(entry),
            Err(_)    => Entry::create_by_url(url)
        }
    }

    pub fn create_by_url(url: String) -> Result<Entry, Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("INSERT INTO entries (url) VALUES ($1) RETURNING id").unwrap();
        for row in stmt.query(&[&url]).unwrap().iter() {
            let entry = Entry {
                      id: row.get(0),
                     url: url,
                   title: None,
             description: None,
              visual_url: None,
                  locale: None,
                  tracks: Vec::new(),
            };
            return Ok(entry);
        }
        Err(Error::Unexpected)
    }

    pub fn add_track(&mut self, track: Track) {
        let conn = conn().unwrap();
        let stmt = conn.prepare("INSERT INTO track_entries (track_id, entry_id)
                                 VALUES ($1, $2)").unwrap();
        stmt.query(&[&track.id, &self.id]).unwrap();
        self.tracks.push(track);
    }

    pub fn save(&self) -> Result<(), Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("UPDATE entries SET url=$2, title=$3, description=$4, visual_url=$5, locale=$6 WHERE id = $1").unwrap();
        let result = stmt.query(&[&self.id,
                                  &self.url,
                                  &self.title,
                                  &self.description,
                                  &self.visual_url,
                                  &self.locale]);
        try!(result);
        Ok(())
    }
}
