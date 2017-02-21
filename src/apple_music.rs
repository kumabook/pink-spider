use scraping::{Html, Selector};
use scraping::element_ref::ElementRef;
use hyper::Client;
use hyper::header::Connection;
use hyper::header::ConnectionOption;
use std::io::Read;
use std::error;
use std::fmt;
use regex::Regex;

static BASE_URL:  &'static str = "http://tools.applemusic.com/embed/v1/";
static MUSIC_URL: &'static str = r#"musicUrl = "([\x00-\x21\x23-\x7F]+)""#; // except \x22(")

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Song {
    pub id:          String,
    pub country:     String,
    pub title:       String,
    pub artwork_url: String,
    pub artist:      String,
    pub audio_url:   String,
    pub music_url:   String,
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Album {
    pub id:           String,
    pub country:      String,
    pub title:        String,
    pub artwork_url:  String,
    pub album_artist: String,
    pub music_url:    String,
    pub genre:        String,
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Playlist {
    pub id:          String,
    pub country:     String,
    pub title:       String,
    pub curator:     String,
    pub description: String,
    pub artwork_url: String,
    pub music_url:   String,
    pub count:       String,
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Track {
    pub title:       String,
    pub artwork_url: String,
    pub artist:      String,
    pub audio_url:   String,
}

#[derive(Debug)]
pub struct ScrapeError {
    reason: String,
}

impl fmt::Display for ScrapeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

type ScrapeResult<T> = Result<T, ScrapeError>;

impl error::Error for ScrapeError {
    fn description(&self) -> &str {
        &self.reason
    }
}

pub fn fetch_song(id: &str, country: &str) -> ScrapeResult<Song> {
    let client = Client::new();
    let url = format!("{}/song/{}?country={}", BASE_URL, id, country);
    let mut res = client.get(&url)
        .header(Connection(vec![ConnectionOption::Close]))
        .send()
        .unwrap();
    if !res.status.is_success() {
        return Err(ScrapeError { reason: "network error".to_string() })
    }
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let fragment = Html::parse_fragment(&body);

    let artwork_url = try!(extract_artwork_url(fragment.clone()));
    let title       = try!(extract_song_title(fragment.clone()));
    let artist      = try!(extract_song_artist(fragment.clone()));
    let audio_url   = try!(extract_audio_url(fragment.clone()));
    let music_url   = try!(extract_music_url(fragment.clone()));
    Ok(Song {
        id:          id.to_string(),
        country:     country.to_string(),
        title:       title,
        artist:      artist,
        artwork_url: artwork_url,
        audio_url:   audio_url,
        music_url:   music_url,
    })
}

pub fn fetch_album(id: &str, country: &str) -> ScrapeResult<Album> {
    let client = Client::new();
    let url = format!("{}/album/{}?country={}", BASE_URL, id, country);
    let mut res = client.get(&url)
        .header(Connection(vec![ConnectionOption::Close]))
        .send()
        .unwrap();
    if !res.status.is_success() {
        return Err(ScrapeError { reason: "network error".to_string() })
    }
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let fragment = Html::parse_fragment(&body);

    let artwork_url  = try!(extract_artwork_url(fragment.clone()));
    let title        = try!(extract_title(fragment.clone()));
    let album_artist = try!(extract_album_artist(fragment.clone()));
    let count        = try!(extract_count(fragment.clone()));
    let music_url    = try!(extract_music_url(fragment.clone()));
    let mut tracks   = Vec::new();
    let tracks_selector = Selector::parse(".track").unwrap();
    for node in fragment.select(&tracks_selector) {
        tracks.push(extract_track(node));
    }
    Ok(Album {
        id:           id.to_string(),
        country:      country.to_string(),
        title:        title,
        album_artist: album_artist,
        artwork_url:  artwork_url,
        music_url:    music_url,
        genre:        count,
    })
}

pub fn fetch_playlist(id: &str, country: &str) -> ScrapeResult<Playlist> {
    let client = Client::new();
    let url = format!("{}/playlist/{}?country={}", BASE_URL, id, country);
    let mut res = client.get(&url)
        .header(Connection(vec![ConnectionOption::Close]))
        .send()
        .unwrap();
    if !res.status.is_success() {
        return Err(ScrapeError { reason: "network error".to_string() })
    }
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let fragment = Html::parse_fragment(&body);

    let artwork_url = try!(extract_artwork_url(fragment.clone()));
    let title       = try!(extract_title(fragment.clone()));
    let description = try!(extract_description(fragment.clone()));
    let curator     = try!(extract_curator(fragment.clone()));
    let count       = try!(extract_count(fragment.clone()));
    let music_url   = try!(extract_music_url(fragment.clone()));
    let mut tracks  = Vec::new();
    let tracks_selector = Selector::parse(".track").unwrap();
    for node in fragment.select(&tracks_selector) {
        let track = extract_track(node);
        tracks.push(track);
    }
    Ok(Playlist {
        id:          id.to_string(),
        country:     country.to_string(),
        title:       title,
        curator:     curator,
        description: description,
        artwork_url: artwork_url,
        music_url:   music_url,
        count:       count,
    })
}

fn extract_music_url(html: Html) -> ScrapeResult<String> {
    let selector = Selector::parse("script").unwrap();
    html.select(&selector)
        .last()
        .and_then(|img| img.text().next())
        .and_then(|script| match Regex::new(MUSIC_URL) {
            Ok(re) => match re.captures(script) {
                Some(cap) => Some(cap[1].to_string()),
                None => None
            },
            Err(_) => None
        })
        .ok_or(ScrapeError { reason: "music url is not found".to_string() })
}

fn extract_artwork_url(html: Html) -> ScrapeResult<String> {
    let selector = Selector::parse("#heroArtImage > img").unwrap();
    html.select(&selector)
        .next()
        .and_then(|img| img.value().attr("src"))
        .map(|url| url.trim().to_string())
        .ok_or(ScrapeError { reason: "artwork url is not found".to_string() })
}

fn extract_audio_url(html: Html) -> ScrapeResult<String> {
    let selector = Selector::parse("#heroArtImage > .song-audio").unwrap();
    html.select(&selector)
        .next()
        .and_then(|img| img.value().attr("data-url"))
        .map(|url| url.trim().to_string())
        .ok_or(ScrapeError { reason: "audio url is not found".to_string() })
}

fn extract_description(html: Html) -> ScrapeResult<String> {
    let selector = Selector::parse("#description").unwrap();
    html.select(&selector)
        .next()
        .and_then(|div| div.text().next())
        .map(|text| text.trim().to_string())
        .ok_or(ScrapeError { reason: "description is not found".to_string() })
}

fn extract_title(html: Html) -> ScrapeResult<String> {
    let selector = Selector::parse(".heroMeta > .title > a").unwrap();
    html.select(&selector)
        .next()
        .and_then(|a| a.text().next())
        .map(|text| text.trim().to_string())
        .ok_or(ScrapeError { reason: "title is not found".to_string() })
}

fn extract_song_title(html: Html) -> ScrapeResult<String> {
    let selector = Selector::parse(".heroMeta > .details > .title-explicit > .title > a").unwrap();
    html.select(&selector)
        .next()
        .and_then(|a| a.text().next())
        .map(|text| text.trim().to_string())
        .ok_or(ScrapeError { reason: "title is not found".to_string() })
}

fn extract_curator(html: Html) -> ScrapeResult<String> {
    let selector = Selector::parse(".heroMeta > .curator > a").unwrap();
    html.select(&selector)
        .next()
        .and_then(|a| a.text().next())
        .map(|text| text.trim().to_string())
        .ok_or(ScrapeError { reason: "curator is not found".to_string() })
}

fn extract_album_artist(html: Html) -> ScrapeResult<String> {
    let selector = Selector::parse(".heroMeta > .album-artist > a").unwrap();
    html.select(&selector)
        .next()
        .and_then(|a| a.text().next())
        .map(|text| text.trim().to_string())
        .ok_or(ScrapeError { reason: "album artist is not found".to_string() })
}

fn extract_song_artist(html: Html) -> ScrapeResult<String> {
    let selector = Selector::parse(".heroMeta > .details > .song-artist > a").unwrap();
    html.select(&selector)
        .next()
        .and_then(|a| a.text().next())
        .map(|text| text.trim().to_string())
        .ok_or(ScrapeError { reason: "song artist is not found".to_string() })
}

fn extract_count(html: Html) -> ScrapeResult<String> {
    let selector = Selector::parse(".heroMeta > .count").unwrap();
    html.select(&selector)
        .next()
        .and_then(|a| a.text().next())
        .map(|text| text.trim().to_string())
        .ok_or(ScrapeError { reason: "count is not found".to_string() })
}

fn extract_track(node: ElementRef) -> Track {
    let img_selector    = Selector::parse(".artworkImage > img").unwrap();
    let title_selector  = Selector::parse(".title").unwrap();
    let artist_selector = Selector::parse(".artist").unwrap();
    let audio_selector  = Selector::parse(".playlist-audio").unwrap();

    let mut title:       String = "".to_string();
    let mut artwork_url: String = "".to_string();
    let mut artist:      String = "".to_string();
    let mut audio_url:   String = "".to_string();
    if let Some(n) = node.select(&title_selector).next() {
        if let Some(text) = n.text().next() {
            title = text.trim().to_string();
        }
    }

    if let Some(n) = node.select(&artist_selector).next() {
        if let Some(text) = n.text().next() {
            artist = text.trim().to_string();
        }
    }

    if let Some(n) = node.select(&img_selector).next() {
        if let Some(url) = n.value().attr("src") {
            artwork_url = url.trim().to_string();
        }
    }

    if let Some(n) = node.select(&audio_selector).next() {
        if let Some(url) = n.value().attr("data-url") {
            audio_url = url.trim().to_string();
        }
    }

    Track {
        title:       title,
        artwork_url: artwork_url,
        artist:      artist,
        audio_url:   audio_url,
    }
}

#[cfg(test)]
mod test {
    use super::fetch_playlist;
    use super::fetch_album;
    use super::fetch_song;
    #[test]
    fn test_fetch_playlist() {
        let playlist = fetch_playlist("pl.2ff0e502db0c44a598a7cb2261a5e6b2", "jp").unwrap();
        assert_eq!(playlist.id, "pl.2ff0e502db0c44a598a7cb2261a5e6b2");
    }
    #[test]
    fn test_fetch_album() {
        let album = fetch_album("1160715126", "jp").unwrap();
        assert_eq!(album.id, "1160715126");
    }
    #[test]
    fn test_fetch_song() {
        let song = fetch_song("1160715431", "jp").unwrap();
        assert_eq!(song.id, "1160715431");
    }
}
