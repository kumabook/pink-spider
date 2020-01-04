extern crate pink_spider;

#[test]
fn it_works() {
  assert!(true);
}

#[test]
fn fetch_apple_music_track() {
    let song = pink_spider::apple_music::fetch_song("jp", "1160715431").unwrap();
    let track = pink_spider::model::Track::from_am_song(&song);
    assert_eq!(track.identifier, "1160715431");
    assert_eq!(track.title, "A Short Film");
    assert_eq!(track.owner_name.unwrap_or_default(), "LILI LIMIT");
    let artists = track.artists.unwrap();
    let artist = &artists[0];
    assert_eq!(artist.name, "LILI LIMIT");
    assert!(artist.clone().artwork_url.unwrap().len() > 0);
}

#[test]
fn fetch_apple_music_album() {
    let am_album = pink_spider::apple_music::fetch_album("jp", "1160715126").unwrap();
    let album = pink_spider::model::Album::from_am_album(&am_album);
    assert_eq!(album.identifier, "1160715126");
    assert_eq!(album.title, "a.k.a");
    assert_eq!(album.owner_name.unwrap_or_default(), "LILI LIMIT");
    let artists = album.artists.unwrap();
    let artist = &artists[0];
    assert_eq!(artist.name, "LILI LIMIT");
    assert!(artist.clone().artwork_url.unwrap().len() > 0);
}

#[test]
fn fetch_spotify_track() {
    let sp_track = pink_spider::spotify::fetch_track("3n3Ppam7vgaVa1iaRUc9Lp").unwrap();
    let track = pink_spider::model::Track::from_sp_track(&sp_track).unwrap();
    assert_eq!(track.identifier, "3n3Ppam7vgaVa1iaRUc9Lp");
    assert_eq!(track.title, "Mr. Brightside");
    assert_eq!(track.owner_name.unwrap_or_default(), "The Killers");
    let artists = track.artists.unwrap();
    let artist = &artists[0];
    assert_eq!(artist.name, "The Killers");
    assert!(artist.clone().artwork_url.unwrap().len() > 0);
}

#[test]
fn fetch_spotify_album() {
    let sp_album = pink_spider::spotify::fetch_album("4OHNH3sDzIxnmUADXzv2kT").unwrap();
    let album = pink_spider::model::Album::from_sp_album(&sp_album);
    assert_eq!(album.identifier, "4OHNH3sDzIxnmUADXzv2kT");
    assert_eq!(album.title, "Hot Fuss");
    assert_eq!(album.owner_name.unwrap_or_default(), "The Killers");
    let artists = album.artists.unwrap();
    let artist = &artists[0];
    assert_eq!(artist.name, "The Killers");
    assert!(artist.clone().artwork_url.unwrap().len() > 0);
    assert!(album.tracks.len() > 0);
}

#[test]
fn fetch_soundcloud_track() {
    let sc_track = pink_spider::soundcloud::fetch_track("371851634").unwrap();
    let track = pink_spider::model::Track::from_sc_track(&sc_track);
    assert_eq!(track.identifier, "371851634");
    assert_eq!(track.title, "Down Wit That");
    assert_eq!(track.owner_name.unwrap_or_default(), "\"Chance The Rapper\"");
    let artists = track.artists.unwrap();
    let artist = &artists[0];
    assert_eq!(artist.name, "\"Chance The Rapper\"");
    assert!(artist.clone().artwork_url.unwrap().len() > 0);
}

#[test]
fn fetch_youtube_track() {
    let yt_video = pink_spider::youtube::fetch_video("Wr5f6hpYxmE").unwrap();
    let track = pink_spider::model::Track::from_yt_video(&yt_video);
    assert_eq!(track.identifier, "Wr5f6hpYxmE");
    assert_eq!(track.title, "Cornelius 『あなたがいるなら』If You're Here");
    assert_eq!(track.owner_name.unwrap_or_default(), "corneliusofficial");
    let artists = track.artists.unwrap();
    let artist = &artists[0];
    assert_eq!(artist.name, "corneliusofficial");
    assert!(artist.clone().artwork_url.unwrap().len() > 0);
}
