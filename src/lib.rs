extern crate hyper;
extern crate hyper_native_tls;
extern crate iron;
extern crate html5ever;
extern crate tendril;
extern crate scraper as scraping;
extern crate regex;
extern crate string_cache;
extern crate url;
extern crate urlencoded;
extern crate uuid;
extern crate postgres;
extern crate chrono;
extern crate queryst;
extern crate toml;
extern crate encoding;

#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

extern crate opengraph;
extern crate feed_rs;

pub use self::model::Feed;
pub use self::model::Track;
pub use self::model::Playlist;
pub use self::model::Album;
pub use self::model::Provider;

pub mod error;
pub mod scraper;
pub mod rss;
pub mod model;
pub mod apple_music;
pub mod itunes;
pub mod youtube;
pub mod soundcloud;
pub mod spotify;
pub mod lemoned;
pub mod get_env;
pub mod http;
