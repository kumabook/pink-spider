use std::collections::BTreeMap;
use rustc_serialize::json::{ToJson, Json};
use error::Error;
use super::conn;

#[derive(Debug, Clone)]
pub struct Feed {
    pub id:           String,
    pub subscribers:  Option<i64>,
    pub title:        Option<String>,
    pub description:  Option<String>,
    pub language:     Option<String>,
    pub velocity:     Option<f64>,
    pub website:      Option<String>,
    pub topics:       Option<Vec<String>>,
    pub status:       Option<String>,
    pub curated:      Option<bool>,
    pub featured:     Option<bool>,
    pub last_updated: Option<i64>,

    pub visual_url:   Option<String>,
    pub icon_url:     Option<String>,
    pub cover_url:    Option<String>,
}

impl ToJson for Feed {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("id".to_string()          , self.id.to_string().to_json());
        d.insert("subscribers".to_string() , self.subscribers.to_json());
        d.insert("title".to_string()       , self.title.to_json());
        d.insert("description".to_string() , self.description.to_json());
        d.insert("language".to_string()    , self.language.to_json());
        d.insert("velocity".to_string()    , self.velocity.to_json());
        d.insert("website".to_string()     , self.website.to_json());
        d.insert("topics".to_string()      , self.topics.to_json());
        d.insert("status".to_string()      , self.status.to_json());
        d.insert("curated".to_string()     , self.curated.to_json());
        d.insert("featured".to_string()    , self.featured.to_json());
        d.insert("last_updated".to_string(), self.last_updated.to_json());
        d.insert("visual_url".to_string()  , self.visual_url.to_json());
        d.insert("icon_url".to_string()    , self.icon_url.to_json());
        d.insert("cover_url".to_string()   , self.cover_url.to_json());
        Json::Object(d)
    }
}

impl Feed {
    pub fn find_by_id(id: String) -> Result<Feed, Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("SELECT id, subscribers, title, description, language, velocity, website, status, curated, featured, last_updated, visual_url, icon_url, cover_url FROM feeds WHERE id = $1").unwrap();
        for row in stmt.query(&[&id]).unwrap().iter() {
            return Ok(Feed {
                    id: row.get(0),
           subscribers: row.get(1),
                 title: row.get(2),
           description: row.get(3),
              language: row.get(4),
              velocity: row.get(5),
               website: row.get(6),
                topics: None,
                status: row.get(7),
               curated: row.get(8),
              featured: row.get(9),
          last_updated: row.get(10),
            visual_url: row.get(11),
              icon_url: row.get(12),
             cover_url: row.get(13),
            });
        }
        return Err(Error::NotFound)
    }

    pub fn find_all() -> Vec<Feed> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("SELECT id, subscribers, title, description, language, velocity, website, status, curated, featured, last_updated, visual_url, icon_url, cover_url FROM feeds").unwrap();
        let mut feeds = Vec::new();
        for row in stmt.query(&[]).unwrap().iter() {
            feeds.push(Feed {
                    id: row.get(0),
           subscribers: row.get(1),
                 title: row.get(2),
           description: row.get(3),
              language: row.get(4),
              velocity: row.get(5),
               website: row.get(6),
                topics: None,
                status: row.get(7),
               curated: row.get(8),
              featured: row.get(9),
          last_updated: row.get(10),
            visual_url: row.get(11),
              icon_url: row.get(12),
             cover_url: row.get(13),
            })
        }
        return feeds
    }

    pub fn create(feed: Feed) -> Result<(), Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare("INSERT INTO feeds (id, u) VALUES ($1)").unwrap();
        let _ = try!(stmt.query(&[
            &feed.id,
            &feed.subscribers,
            &feed.title,
            &feed.description,
            &feed.language,
            &feed.velocity,
            &feed.website,
            &feed.topics,
            &feed.status,
            &feed.curated,
            &feed.featured,
            &feed.last_updated,
            &feed.visual_url,
            &feed.icon_url,
            &feed.cover_url,
        ]));
        Ok(())
    }
}
