extern crate chrono;
extern crate pink_spider;

use std::time::Instant;
use chrono::Duration;
use pink_spider::model::Playlist;

pub fn main() {
    println!("Start crawling playlists...");
    let now = Instant::now();
    let mut playlists = Playlist::find_actives();
    println!("{} playlists", playlists.len());
    let total = playlists.len();
    let mut index = 0;
    for playlist in playlists.iter_mut() {
        println!("[{}/{}] Fetch latest tracks of [{}] {}: {}",
                 index,
                 total,
                 playlist.provider,
                 playlist.id,
                 playlist.title);
        let tracks = match playlist.fetch_tracks() {
            Ok(tracks) => tracks,
            Err(e)     => {
                println!("{}", e);
                vec![]
            }
        };
        println!("{} tracks are added", tracks.len());
        index += 1;
    }

    println!("Complete crawling playlists... total {} ms",
             Duration::from_std(now.elapsed()).unwrap().num_milliseconds());
}
