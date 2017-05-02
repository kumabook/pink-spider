#[macro_use]
extern crate pink_spider;
use pink_spider::youtube;
use pink_spider::soundcloud;
use pink_spider::spotify;
use pink_spider::model::{Model, Enclosure, Playlist, Provider};
use pink_spider::model::{conn};
use std::time::Duration;
use std::thread;

pub fn main() {
    let conn = conn().unwrap();
    let stmt = conn.prepare(
        &format!("SELECT {} FROM playlists WHERE playlists.provider = 'Spotify' AND playlists.description IS NULL ORDER BY playlists.published_at DESC", Playlist::props_str(""))).unwrap();
    let rows = stmt.query(&[]).unwrap();
    let playlists = Playlist::rows_to_items(rows);
    println!("len {}\n", playlists.len());
    for mut playlist in playlists {
        thread::sleep(Duration::from_millis(500));
        playlist.fetch_props();
        println!("description {:?}", playlist.description);
        match playlist.save() {
            Ok(_) => {
                print!("playlist id: {} {}:{} state: {:?} is updated\n", playlist.id, playlist.provider, playlist.identifier, playlist.state);
            },
            Err(e) => {
                print!("Failed to update playlist id: {} {}:{}\n", playlist.id, playlist.provider, playlist.identifier);
                print!("{}\n", e);
            },
        }
    }
}
