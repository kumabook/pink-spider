pub enum Provider {
    YouTube,
    SoundCloud,
    Raw
}
pub struct Track {
    pub provider:  Provider,
    pub title:     String,
    pub url:       String,
    pub serviceId: String
}

pub struct Playlist {
    pub title:  String,
    pub tracks: Vec<Track>,
}

