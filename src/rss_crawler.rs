extern crate pink_spider;
extern crate feed_rs;
extern crate hyper;
extern crate chrono;
extern crate serde_json;

use std::time::Instant;
use chrono::Duration;
use pink_spider::model::{Model, Feed};

pub fn main() {
    println!("[rss_cralwer] Start crawling...");
    let now = Instant::now();
    let mut page  = 0;
    let per_page  = 10;
    let mut feeds = Feed::find(0, 0, None);
    let total = feeds.total;
    let mut index = 0;
    println!("[rss_cralwer] {} feeds", total);
    while feeds.page * feeds.per_page < total {
        feeds = Feed::find(page, per_page, None);
        for mut feed in feeds.items {
            println!("[rss_cralwer][{}/{}] Crawl {} ", index, total, feed.url);
            match feed.crawl() {
                Ok(entries) => {
                    println!("[rss_cralwer] Found {} entries", entries.len());
                    println!("[rss_cralwer] Found {} tracks",
                             entries.iter().fold(0, |sum, e| sum + e.tracks.len()));
                    println!("[rss_cralwer] Found {} albums",
                             entries.iter().fold(0, |sum, e| sum + e.albums.len()));
                    println!("[rss_cralwer] Found {} playlists",
                             entries.iter().fold(0, |sum, e| sum + e.playlists.len()));
                },
                Err(e) => {
                    println!("[rss_cralwer] Failed to crawl {}: {:?}", feed.url, e);
                },
            }
            index += 1;
        }
        page += 1;
    }
    println!("[rss_cralwer] Complete crawling... total {} ms",
             Duration::from_std(now.elapsed()).unwrap().num_milliseconds());
}
