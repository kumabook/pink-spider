pub use self::track::Track;
pub use self::playlist::Playlist;
pub use self::album::Album;
pub use self::artist::Artist;
pub use self::enclosure::Enclosure;
pub use self::provider::Provider;
pub use self::entry::Entry;
pub use self::feed::Feed;

mod track;
mod playlist;
mod album;
mod artist;
mod entry;
mod feed;
mod provider;
mod state;
mod enclosure;
pub mod open_graph;

use rustc_serialize::json::{ToJson, Json};
use std;
use std::collections::BTreeMap;
use uuid::Uuid;
use postgres;
use postgres::{Connection, TlsMode};
use postgres::error::ConnectError;
use std::env;
use error::Error;

static DEFAULT_DATABASE_URL: &'static str = "postgres://postgres:postgres@localhost/pink_spider_development";

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct PaginatedCollection<I> {
    pub page:     i64,
    pub per_page: i64,
    pub total:    i64,
    pub items:    Vec<I>,
}

impl<I: ToJson> ToJson for PaginatedCollection<I> {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        let items = Json::Array(self.items.iter().map(|x| x.to_json()).collect());
        d.insert("page".to_string()    , self.page.to_string().to_json());
        d.insert("per_page".to_string(), self.per_page.to_json());
        d.insert("total".to_string()   , self.total.to_json());
        d.insert("items".to_string()   , items);
        Json::Object(d)
    }
}


pub fn conn() -> Result<Connection, ConnectError> {
    let opt_url = env::var("DATABASE_URL");
    match opt_url {
        Ok(url) =>
            Connection::connect(url.trim(), TlsMode::None),
        Err(_)  =>
            Connection::connect(DEFAULT_DATABASE_URL, TlsMode::None)
    }
}

pub trait Model where Self: std::marker::Sized + ToJson + Clone {
    fn table_name() -> String;
    fn props_str(prefix: &str) -> String;
    fn rows_to_items(rows: postgres::rows::Rows) -> Vec<Self>;
    fn find_by_id(id: &str) -> Result<Self, Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(
            &format!("SELECT {} FROM {} WHERE id = $1",
                     Self::props_str(""),
                     Self::table_name())).unwrap();
        let uuid   = try!(Uuid::parse_str(id).map_err(|_| Error::Unprocessable));
        let rows   = stmt.query(&[&uuid]).unwrap();
        let items = Self::rows_to_items(rows);
        if items.len() > 0 {
            return Ok(items[0].clone());
        }
        return Err(Error::NotFound)
    }
    fn find(page: i64, per_page: i64) -> PaginatedCollection<Self> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(&format!("SELECT {}  FROM {}
                                            ORDER BY updated_at DESC
                                            LIMIT $2 OFFSET $1",
                                         Self::props_str(""),
                                         Self::table_name())).unwrap();
        let offset = page * per_page;
        let rows   = stmt.query(&[&offset, &per_page]).unwrap();
        let items = Self::rows_to_items(rows);
        let mut total: i64 = 0;
        let sql = format!("SELECT COUNT(*) FROM {}", Self::table_name());
        for row in conn.query(&sql, &[]).unwrap().iter() {
            total = row.get(0);
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
        let conn = try!(conn());
        let stmt = try!(conn.prepare(&format!("SELECT {} FROM {} WHERE id IN ({})",
                                              Self::props_str(""),
                                              Self::table_name(),
                                              ids.join(","))));
        let rows = try!(stmt.query(&[]));
        Ok(Self::rows_to_items(rows))
    }
    fn create(&self) -> Result<Self, Error>;
    fn save(&mut self) -> Result<(), Error>;
}
