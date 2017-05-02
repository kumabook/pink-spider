use error::Error;
use super::conn;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
