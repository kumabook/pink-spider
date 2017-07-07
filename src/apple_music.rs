use scraping::{Html, Selector};
use scraping::element_ref::ElementRef;
use hyper::header::Connection;
use hyper::header::ConnectionOption;
use std::io::Read;
use std::error;
use std::fmt;
use regex::Regex;
use url::Url;
use queryst::parse;
use http;

static BASE_URL:  &'static str = "http://tools.applemusic.com/embed/v1/";
static MUSIC_URL: &'static str = r#"musicUrl = "([\x00-\x21\x23-\x7F]+)""#; // except \x22(")

static ALBUM_LINK:    &'static str = r"itunes.apple.com/([a-zA-Z0-9_-]+)/album/([a-zA-Z0-9_-]+)/id([a-zA-Z0-9_-]+)";
static PLAYLIST_LINK: &'static str = r"itunes.apple.com/([a-zA-Z0-9_-]+)/playlist/([^/]+)/idpl.([a-zA-Z0-9_-]+)";

pub static SONG_URL:      &'static str = r"tools.applemusic.com/embed/v1/song/([a-zA-Z0-9_-]+)";
pub static ALBUM_URL:     &'static str = r"tools.applemusic.com/embed/v1/album/([a-zA-Z0-9_-]+)";
pub static PLAYLIST_URL:  &'static str = r"tools.applemusic.com/embed/v1/playlist/pl.([a-zA-Z0-9_-]+)";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Song {
    pub id:          String,
    pub country:     String,
    pub title:       String,
    pub artwork_url: String,
    pub artist:      String,
    pub audio_url:   String,
    pub music_url:   String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Album {
    pub id:           String,
    pub country:      String,
    pub title:        String,
    pub artwork_url:  String,
    pub album_artist: String,
    pub music_url:    String,
    pub genre:        String,
    pub tracks:       Option<Vec<Track>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

fn url_param(url_str: &str, key: &str) -> Option<String> {
    let url_str = if url_str.starts_with("http") || url_str.starts_with("https") {
        url_str.to_string()
    } else {
        "https://".to_string() + url_str
    };
    let url = Url::parse(&url_str);
    if !url.is_ok() {
        return None;
    }
    let params = url.unwrap().query()
        .map(|q| parse(&q))
        .and_then(|r| r.ok());
    if let Some(params) = params {
        params.as_object()
            .and_then(|params| params.get(key))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    } else {
        None
    }
}

pub fn country(url: &str) -> String {
    if let Some(url) = url_param(url, "country") {
        url
    } else if let Some(url) = parse_url_as_playlist(url).map(|v| v.0) {
        url
    } else if let Some(url) = parse_url_as_album(url).map(|v| v.0) {
        url
    } else {
        "us".to_string()
    }
}

pub fn parse_url_as_album(value: &str) -> Option<(String, String, String, Option<String>)> {
    parse_url(&value, ALBUM_LINK)
}

pub fn parse_url_as_playlist(value: &str) -> Option<(String, String, String, Option<String>)> {
    parse_url(&value, PLAYLIST_LINK)
}

pub fn parse_url(value: &str, regex_str: &str) -> Option<(String, String, String, Option<String>)> {
    match Regex::new(regex_str) {
        Ok(re) => match re.captures(value) {
            Some(cap) => {
                let country: String = cap[1].to_string();
                let name:    String = cap[2].to_string();
                let id:      String = cap[3].to_string();
                return Some((country, name, id, url_param(value, "i")));
            },
            None => None
        },
        Err(_) => None
    }
}

pub fn fetch_song(id: &str, country: &str) -> ScrapeResult<Song> {
    let url = format!("{}/song/{}?country={}", BASE_URL, id, country);
    let mut res = http::client().get(&url)
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
    let url = format!("{}/album/{}?country={}", BASE_URL, id, country);
    let mut res = http::client().get(&url)
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
        tracks:       Some(tracks),
    })
}

pub fn fetch_playlist(id: &str, country: &str) -> ScrapeResult<Playlist> {
    let url = format!("{}/playlist/pl.{}?country={}", BASE_URL, id, country);
    let mut res = http::client().get(&url)
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
    let mut urls = html.select(&selector)
        .map(|tag|
             tag.text().next().and_then(|script| match Regex::new(MUSIC_URL) {
                 Ok(re) => match re.captures(script) {
                     Some(cap) => Some(cap[1].to_string()),
                     None => None
                 },
                 Err(_) => None
             }))
        .filter(|ref url| url.is_some())
        .map(|url| url.unwrap());
    if let Some(url) = urls.next() {
        Ok(url)
    } else {
        Err(ScrapeError { reason: "music url is not found".to_string() })
    }
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
        .or(Some("".to_string()))
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
    use super::*;
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
    #[test]
    fn test_parse_url_as_album() {
        let album_song_link = "https://geo.itunes.apple.com/us/album/last-nite/id266376953?i=266377010&mt=1&app=music";
        match parse_url_as_album(album_song_link) {
            Some((country, name, id, song_id)) => {
                assert_eq!(&country         , "us");
                assert_eq!(&name            , "last-nite");
                assert_eq!(&id              , "266376953");
                assert_eq!(&song_id.unwrap(), "266377010");
            },
            None => assert!(false),
        }
    }
    #[test]
    fn test_parse_url_as_playlist() {
        let playlist_link = "https://itunes.apple.com/us/playlist/the-strokes-essentials/idpl.3a7a911b00c048ebba63b651935a241a?mt=1&app=music";
        match parse_url_as_playlist(playlist_link) {
            Some((country, name, id, song_id)) => {
                assert_eq!(&country         , "us");
                assert_eq!(&name            , "the-strokes-essentials");
                assert_eq!(&id              , "3a7a911b00c048ebba63b651935a241a");
                assert!(song_id.is_none());
            },
            None => assert!(false),
        }
    }
}
