extern crate pink_spider;
use pink_spider::spotify;
use pink_spider::apple_music;
use pink_spider::apple_music::country;
use pink_spider::model::{Model, Album, Provider};
use pink_spider::model::State;

pub fn main() {
    let mut albums = Album::find_all();
    println!("len {}\n", albums.len());
    for mut album in albums.iter_mut().filter(|a| a.state == State::Alive) {
        let album = match album.provider {
            Provider::Spotify => match spotify::fetch_album(&album.identifier) {
                Ok(sp_album) => album.update_with_sp_album(&sp_album),
                Err(e)       => {
                    print!("{}\n", e);
                    album.disable()
                },
            },
            Provider::AppleMusic => match apple_music::fetch_album(&country(&album.url),
                                                                   &album.identifier) {
                Ok(am_album) => album.update_with_am_album(&am_album),
                Err(e)       => {
                    print!("{}\n", e);
                    album.disable()
                },
            },
            _ => &mut album,
        };
        match album.save() {
            Ok(_) => {
                print!("[{:?}] album id: {} {} {} {} {} is updated\n",
                       album.state,
                       album.id, album.provider, album.identifier,
                       country(&album.url),
                       album.title);
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
