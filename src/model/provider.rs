use std::fmt;
use rustc_serialize::json::{ToJson, Json};

#[derive(Debug, Copy, Clone, RustcDecodable, RustcEncodable)]
pub enum Provider {
    AppleMusic,
    YouTube,
    SoundCloud,
    Spotify,
    Raw
}

impl PartialEq for Provider {
    fn eq(&self, p: &Provider) -> bool {
        match *self {
            Provider::AppleMusic => match *p { Provider::AppleMusic => true, _ => false },
            Provider::YouTube    => match *p { Provider::YouTube    => true, _ => false },
            Provider::SoundCloud => match *p { Provider::SoundCloud => true, _ => false },
            Provider::Spotify    => match *p { Provider::Spotify    => true, _ => false },
            Provider::Raw        => match *p { Provider::Raw        => true, _ => false }
        }
    }
}

impl Provider {
    fn to_string(&self) -> String {
        match *self {
            Provider::AppleMusic => "AppleMusic",
            Provider::YouTube    => "YouTube",
            Provider::SoundCloud => "SoundCloud",
            Provider::Spotify    => "Spotify",
            Provider::Raw        => "Raw",
        }.to_string()
    }
    pub fn new(str: String) -> Provider {
        match str.as_ref() {
            "AppleMusic" => Provider::AppleMusic,
            "applemusic" => Provider::AppleMusic,
            "YouTube"    => Provider::YouTube,
            "youtube"    => Provider::YouTube,
            "SoundCloud" => Provider::SoundCloud,
            "soundcloud" => Provider::SoundCloud,
            "Spotify"    => Provider::Spotify,
            "spotify"    => Provider::Spotify,
            _            => Provider::Raw,
        }
    }
}

impl ToJson for Provider {
    fn to_json(&self) -> Json {
        self.to_string().to_json()
    }
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
