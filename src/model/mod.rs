pub use self::track::{Track, Provider, Playlist};
pub use self::entry::Entry;

mod track;
mod entry;
pub mod open_graph;

use postgres::{Connection, SslMode};
use postgres::error::ConnectError;
use std::env;


static DEFAULT_DATABASE_URL: &'static str = "postgres://kumabook@localhost/pink_spider_development_master";


pub fn conn() -> Result<Connection, ConnectError> {
    let opt_url = env::var("DATABASE_URL");
    match opt_url {
        Ok(url) =>
            Connection::connect(url.trim(), SslMode::None),
        Err(_)  =>
            Connection::connect(DEFAULT_DATABASE_URL, SslMode::None)
    }
}
