
extern crate pink_spider;
extern crate feed_rs;
extern crate hyper;
extern crate chrono;
extern crate serde_json;

use std::time::Instant;
use chrono::Duration;
use pink_spider::model::{Model, Feed};

pub fn main() {
    println!("Start crawling...");
    let mut feed = Feed::find_by_url("http://media.typica.mu/?feed=rss2").unwrap();
    match feed.crawl() {
        Ok(entries) => {
            println!("Found {} entries", entries.len());
            println!("Found {} tracks",
                     entries.iter().fold(0, |sum, e| sum + e.tracks.len()));
            println!("Found {} albums",
                     entries.iter().fold(0, |sum, e| sum + e.albums.len()));
            println!("Found {} playlists",
                     entries.iter().fold(0, |sum, e| sum + e.playlists.len()));
        },
        Err(e) => {
            println!("Failed to crawl {}: {:?}", feed.url, e);
        },
    }
    println!("Complete crawling... ");
}
