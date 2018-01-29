extern crate pink_spider;
use pink_spider::spotify;
use pink_spider::apple_music;
use pink_spider::apple_music::country;
use pink_spider::model::{Model, Album, Provider};
use pink_spider::model::{conn};
use std::time::Duration;
use std::thread;

pub fn main() {
    let conn = conn().unwrap();
    let stmt = conn.prepare(
        &format!("SELECT {} FROM albums WHERE albums.state = 'alive' ORDER BY albums.published_at DESC", Album::props_str(""))).unwrap();
    let rows = stmt.query(&[]).unwrap();
    let albums = Album::rows_to_items(rows);
    println!("len {}\n", albums.len());
    for mut album in albums {
        thread::sleep(Duration::from_millis(100));
        let album = match album.provider {
            Provider::Spotify => match spotify::fetch_album(&album.identifier) {
                Ok(sp_album) => album.update_with_sp_album(&sp_album),
                Err(_)       => album.disable(),
            },
            Provider::AppleMusic => match apple_music::fetch_album(&country(&album.url),
                                                                   &album.identifier) {
                Ok(am_album) => album.update_with_am_album(&am_album),
                Err(_)       => album.disable(),
            },
            _ => &mut album,
        };
        match album.save() {
            Ok(_) => {
                print!("album id: {} {} {} {} {} state: {:?} is updated\n",
                       album.id, album.provider, album.identifier,
                       country(&album.url),
                       album.title, album.state);
            },
            Err(e) => {
                print!("Failed to update album id: {} {} {} {} {}\n",
                       album.id, album.provider,
                       country(&album.url),
                       album.title, album.identifier);
                print!("{}\n", e);
            },
        }
    }
}
