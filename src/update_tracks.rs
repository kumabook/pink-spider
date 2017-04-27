#[macro_use]
extern crate pink_spider;
use pink_spider::youtube;
use pink_spider::soundcloud;
use pink_spider::spotify;
use pink_spider::model::{Model, Track, Provider};
use pink_spider::model::{conn};
use std::time::Duration;
use std::thread;

pub fn main() {
    let conn = conn().unwrap();
    let stmt = conn.prepare(
        &format!("SELECT {} FROM tracks WHERE tracks.owner_id IS NULL AND tracks.state = 'alive' ORDER BY tracks.published_at DESC", Track::props_str(""))).unwrap();
    let rows = stmt.query(&[]).unwrap();
    let tracks = Track::rows_to_items(rows);
    println!("len {}\n", tracks.len());
    for mut track in tracks {
        thread::sleep(Duration::from_millis(500));
        let track = match track.provider {
            Provider::YouTube => match youtube::fetch_video(&track.identifier) {
                Ok(video) => track.update_with_yt_video(&video),
                Err(_)    => track.disable(),
            },
            Provider::SoundCloud => match soundcloud::fetch_track(&track.identifier) {
                Ok(sc_track) => track.update_with_sc_track(&sc_track),
                Err(_)       => track.disable(),
            },
            Provider::Spotify => match spotify::fetch_track(&track.identifier) {
                Ok(sp_track) => track.update_with_sp_track(&sp_track),
                Err(_)       => track.disable(),
            },
            _ => &mut track,
        };
        match track.save() {
            Ok(_) => {
                print!("track id: {} {}:{} state: {:?} is updated\n", track.id, track.provider, track.identifier, track.state);
            },
            Err(e) => {
                print!("Failed to update track id: {} {}:{}\n", track.id, track.provider, track.identifier);
                print!("{}\n", e);
            },
        }
    }
}
