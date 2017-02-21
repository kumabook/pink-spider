extern crate hyper;
extern crate iron;
extern crate rustc_serialize;
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

#[macro_use]
extern crate lazy_static;

pub use self::model::Feed;
pub use self::model::Track;
pub use self::model::Provider;
pub use self::model::open_graph;

pub mod error;
pub mod scraper;
pub mod model;
pub mod apple_music;
pub mod itunes;
pub mod youtube;
pub mod soundcloud;
pub mod spotify;
