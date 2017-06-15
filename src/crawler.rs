extern crate pink_spider;
extern crate feed_rs;
extern crate hyper;
extern crate hyper_native_tls;
extern crate chrono;
extern crate serde_json;

use pink_spider::model::{Model, Feed};

pub fn main() {
    println!("Start crawling...");

    let mut page  = 0;
    let per_page  = 10;
    let mut feeds = Feed::find(0, 0);
    let total = feeds.total;
    println!("{} feeds", total);
    while feeds.page * feeds.per_page < total {
        feeds = Feed::find(page, per_page);
        for mut feed in feeds.items {
            println!("Crawl {} ", feed.url);
            if let Ok(entries) = feed.crawl() {
                println!("Found {} entries", entries.len());
                println!("Found {} tracks",
                         entries.iter().fold(0, |sum, e| sum + e.tracks.len()));
                println!("Found {} albums",
                         entries.iter().fold(0, |sum, e| sum + e.albums.len()));
                println!("Found {} playlists",
                         entries.iter().fold(0, |sum, e| sum + e.playlists.len()));
            }
        }
        page += 1;
    }
    println!("Complete crawling...");
}
