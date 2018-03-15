pub use self::track::Track;
pub use self::playlist::Playlist;
pub use self::album::Album;
pub use self::artist::Artist;
pub use self::enclosure::Enclosure;
pub use self::provider::Provider;
pub use self::entry::Entry;
pub use self::playlist_track::PlaylistTrack;
pub use self::feed::Feed;
pub use self::state::State;

mod track;
mod playlist;
mod album;
mod artist;
mod entry;
mod feed;
mod playlist_track;
mod provider;
mod state;
mod enclosure;

use std;
use uuid::Uuid;
use postgres;
use postgres::{Connection, TlsMode};
use std::env;
use error::Error;
use serde::Serialize;
use serde::Deserialize;
use params;

static DEFAULT_DATABASE_URL: &'static str = "postgres://postgres:postgres@localhost/pink_spider_development";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaginatedCollection<I> {
    pub page:     i64,
    pub per_page: i64,
    pub total:    i64,
    pub items:    Vec<I>,
}

pub enum FilterType {
    Equals,
    Contains,
    GreaterThan,
    LessThan,
    GreaterThanOrEquals,
    LessThanOrEquals,
}

pub enum Value {
    String(String),
    Int(i64),
}

pub struct Filter<'a> {
    pub filter_type: FilterType,
    pub field:       &'a str,
    pub value:       &'a postgres::types::ToSql,
}

impl<'a> Filter<'a> {
    pub fn to_query(&self, num: i32) -> String {
        let comparison = match self.filter_type {
            FilterType::Equals              => "=",
            FilterType::Contains            => "ILIKE",
            FilterType::GreaterThan         => ">",
            FilterType::LessThan            => "<",
            FilterType::GreaterThanOrEquals => ">=",
            FilterType::LessThanOrEquals    => ">=",
        };
        format!("WHERE {} {} ${}", self.field, comparison, num)
    }
}

pub fn conn() -> Result<Connection, postgres::error::Error> {
    let opt_url = env::var("DATABASE_URL");
    match opt_url {
        Ok(url) =>
            Connection::connect(url.trim(), TlsMode::None),
        Err(_)  =>
            Connection::connect(DEFAULT_DATABASE_URL, TlsMode::None)
    }
}

pub trait Model<'a> where Self: std::marker::Sized + Serialize + Deserialize<'a> + Clone {
    fn table_name() -> String;
    fn props_str(prefix: &str) -> String;
    fn row_to_item(rows: postgres::rows::Row) -> Self;
    fn rows_to_items(rows: postgres::rows::Rows) -> Vec<Self> {
        let mut items = Vec::new();
        for row in rows.iter() {
            items.push(Self::row_to_item(row))
        }
        items
    }
    fn set_relations(_items: &mut Vec<Self>) -> Result<(), Error> {
        Ok(())
    }
    fn find_by_id(id: &str) -> Result<Self, Error> {
        let conn = conn()?;
        let stmt = conn.prepare(
            &format!("SELECT {} FROM {} WHERE id = $1",
                     Self::props_str(""),
                     Self::table_name()))?;
        let uuid   = Uuid::parse_str(id).map_err(|_| Error::Unprocessable)?;
        let rows   = stmt.query(&[&uuid])?;
        let items = Self::rows_to_items(rows);
        if items.len() > 0 {
            return Ok(items[0].clone());
        }
        return Err(Error::NotFound)
    }
    fn find(page: i64, per_page: i64, filter: Option<Filter>) -> PaginatedCollection<Self> {
        let conn = conn().unwrap();
        let offset = page * per_page;
        let stmt;
        let rows = if let Some(ref filter) = filter {
            stmt = conn.prepare(&format!("SELECT {} FROM {}
                                            {}
                                            ORDER BY updated_at DESC
                                            LIMIT $2 OFFSET $1",
                                         Self::props_str(""),
                                         Self::table_name(),
                                         filter.to_query(3),
            )).unwrap();
            stmt.query(&[&offset, &per_page, filter.value]).unwrap()
        } else {
            stmt = conn.prepare(&format!("SELECT {} FROM {}
                                            ORDER BY updated_at DESC
                                            LIMIT $2 OFFSET $1",
                                         Self::props_str(""),
                                         Self::table_name())).unwrap();
            stmt.query(&[&offset, &per_page]).unwrap()
        };
        let items = Self::rows_to_items(rows);
        let mut total: i64 = 0;
        if let Some(ref filter) = filter {
            let stmt = conn.prepare(&format!("SELECT COUNT(*) FROM {} {}",
                                            Self::table_name(),
                                            filter.to_query(1))).unwrap();
            for row in stmt.query(&[filter.value]).unwrap().iter() {
                total = row.get(0);
            }
        } else {
            let sql = format!("SELECT COUNT(*) FROM {}", Self::table_name());
            for row in conn.query(&sql, &[]).unwrap().iter() {
                total = row.get(0);
            }
        }
        PaginatedCollection {
            page:     page,
            per_page: per_page,
            total:    total,
            items:    items,
        }
    }
    fn mget(ids: Vec<Uuid>) -> Result<Vec<Self>, Error> {
        let ids: Vec<String> = ids.iter()
                                  .map(|id| format!("'{}'", id.to_string()))
                                  .collect();
        let conn = conn()?;
        let stmt = conn.prepare(&format!("SELECT {} FROM {} WHERE id IN ({})",
                                              Self::props_str(""),
                                              Self::table_name(),
                                              ids.join(",")))?;
        let rows = stmt.query(&[])?;
        let mut items = Self::rows_to_items(rows);
        Self::set_relations(&mut items)?;
        Ok(items)
    }
    fn create(&self) -> Result<Self, Error>;
    fn save(&mut self) -> Result<(), Error>;
    fn update_attributes(&mut self, &params::Map) -> &mut Self {
        self
    }
}
