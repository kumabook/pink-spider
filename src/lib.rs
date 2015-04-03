#![feature(rustc_private, core)]
pub use self::model::Track;
pub use self::model::Playlist;
pub use self::model::Provider;

pub mod scraper;
pub mod model;
