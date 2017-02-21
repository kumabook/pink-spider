pub use self::track::{Track, Playlist};
pub use self::provider::Provider;
pub use self::entry::Entry;
pub use self::feed::Feed;

mod track;
mod entry;
mod feed;
mod provider;
mod state;
pub mod open_graph;

use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use postgres::{Connection, TlsMode};
use postgres::error::ConnectError;
use std::env;


static DEFAULT_DATABASE_URL: &'static str = "postgres://pink_spider:pink_spider@localhost/pink_spider_development";

#[derive(Debug, Clone)]
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
