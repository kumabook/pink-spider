extern crate chrono;
extern crate pink_spider;

use std::time::Instant;
use chrono::Duration;
use pink_spider::model::Playlist;

pub fn main() {
    println!("Start crawling playlists...");
    let now = Instant::now();
    let mut playlists = Playlist::find_actives();
    println!("[pl_cralwer]{} playlists", playlists.len());
    let total = playlists.len();
    let mut index = 0;
    for playlist in playlists.iter_mut() {
        println!("[pl_cralwer][{}/{}] Fetch latest tracks of [{}] {}: {}",
                 index,
                 total,
                 playlist.provider,
                 playlist.id,
                 playlist.title);
        let tracks = match playlist.fetch_tracks() {
            Ok(tracks) => tracks,
            Err(e)     => {
                println!("[pl_cralwer]{}", e);
                vec![]
            }
        };
        println!("[pl_cralwer]{} tracks are added", tracks.len());
        index += 1;
    }

    println!("[pl_cralwer]Complete crawling playlists... total {} ms",
             Duration::from_std(now.elapsed()).unwrap().num_milliseconds());
}
