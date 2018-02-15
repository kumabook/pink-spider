extern crate pink_spider;

use pink_spider::apple_music;
use pink_spider::apple_music::country;
use pink_spider::model::{Track, Album, Artist, Provider};
use pink_spider::model::Enclosure;
use pink_spider::model::State;
use pink_spider::model::Model;

pub fn main() {
    let mut tracks = Track::find_by_provider(&Provider::AppleMusic);
    for track in tracks.iter_mut().filter(|a| a.state == State::Alive) {
        println!("update {:?} {:?}", track.identifier, track.title);
        match apple_music::fetch_song(&country(&track.url),
                                      &track.identifier) {
            Ok(am_song) => track.update_with_am_song(&am_song),
            Err(_)       => track.disable(),
        };
        let _ = track.save();
    }

    update_artists()
}

pub fn update_artists() {
    let artists = Artist::find_all();
    println!("len {}\n", artists.len());
    for mut artist in artists {
        println!("{:?} {:?} {:?}", artist.identifier, artist.provider, artist.name);
        match artist.provider {
            Provider::AppleMusic => update_apple_music(&mut artist),
            Provider::Spotify => update_artist(&mut artist),
            Provider::YouTube => update_artist(&mut artist),
            Provider::SoundCloud => update_artist(&mut artist),
            _ => (),
        }
    }
}

pub fn update_artist(artist: &mut Artist) {
    let _ = artist.fetch_props();
}

pub fn update_apple_music(artist: &mut Artist) {
    let country = country_of_am_artist(artist);
    match apple_music::search_artists(&country, &artist.name) {
        Ok(items) => {
            println!("  {:?} {:?} {}", artist.identifier, country, items.len());
            if items.len() == 1 {
                update_apple_music_artist(artist, &country, &items[0].id);
                //                    println!("Updated {:?}", artist);
                return
            }
            let items = items.iter().filter(|item| {
                item.attributes.name.to_lowercase() == artist.name.to_lowercase()
            }).collect::<Vec<&apple_music::Artist>>();
            if items.len() > 0 {
                update_apple_music_artist(artist, &country, &items[0].id);
                println!("  Updated {:?}", artist.identifier);
                return
            }
            println!("  Not updated {:?}", artist.identifier);
        },
        Err(_) => {
            println!("{:?} {:?} {}", artist.identifier, country, 0);
        },
    }
}

pub fn update_apple_music_artist(artist: &mut Artist, country: &str, id: &str) {
    artist.identifier = id.to_string();
    match apple_music::fetch_artist(country, id) {
        Ok(item) => artist.update_with_am_artist(&item),
        Err(e)   => {
            println!("{:?}", e);
            artist
        },
    };
}

pub fn country_of_am_artist(artist: &Artist) -> String {
    let tracks = Track::find_by_artist(artist.id);
    let albums = Album::find_by_artist(artist.id);
    if tracks.len() > 0 {
        return apple_music::country(&tracks[0].url);
    }
    if albums.len() > 0 {
        return apple_music::country(&albums[0].url);
    }
    println!("-- not found {:?}", artist.id);
    "us".to_string()
}
