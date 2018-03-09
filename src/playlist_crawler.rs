extern crate pink_spider;

use pink_spider::model::Playlist;

pub fn main() {
    println!("Start crawling playlists...");

    let mut playlists = Playlist::find_actives();
    println!("{} playlists", playlists.len());
    for playlist in playlists.iter_mut() {
        println!("Fetch latest tracks of [{}] {}: {}",
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
    }

    println!("Complete crawling playlists...");
}
