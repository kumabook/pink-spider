use hyper::header::{
    Headers,
    Authorization,
    Bearer,
    Connection,
};
use std::io::Read;
use regex::Regex;
use serde_json;
use get_env;
use url::Url;
use queryst::parse;
use http;

static BASE_URL:  &'static str = "https://api.music.apple.com/v1";
static THUMBNAIL_SIZE: &'static str = "300";
static ARTWORK_SIZE: &'static str = "640";


static ALBUM_LINK:    &'static str = r"itunes.apple.com/([a-zA-Z0-9_-]+)/album/([a-zA-Z0-9_-]+)/id([a-zA-Z0-9_-]+)";
static PLAYLIST_LINK: &'static str = r"itunes.apple.com/([a-zA-Z0-9_-]+)/playlist/([^/]+)/idpl.([a-zA-Z0-9_-]+)";

pub static SONG_URL:      &'static str = r"tools.applemusic.com/embed/v1/song/([a-zA-Z0-9_-]+)";
pub static ALBUM_URL:     &'static str = r"tools.applemusic.com/embed/v1/album/([a-zA-Z0-9_-]+)";
pub static PLAYLIST_URL:  &'static str = r"tools.applemusic.com/embed/v1/playlist/pl.([a-zA-Z0-9_-]+)";

lazy_static! {
    static ref DEVELOPER_TOKEN: String = {
        get_env::var("APPLE_MUSIC_DEVELOPER_TOKEN").unwrap_or("".to_string())
    };
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoRelations {}

pub type Genre = Resource<GenreAttributes, NoRelations>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenreAttributes {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Artwork {
    pub width:      i32,
    pub height:     i32,
    pub url:        String,
    pub bg_color:    Option<String>,
    pub text_color1: Option<String>,
    pub text_color2: Option<String>,
    pub text_color3: Option<String>,
    pub text_color4: Option<String>,
}

impl Artwork {
    pub fn get_thumbnail_url(&self) -> String {
        self.url
            .replace("{w}", THUMBNAIL_SIZE)
            .replace("{h}", THUMBNAIL_SIZE)
    }
    pub fn get_artwork_url(&self) -> String {
        self.url
            .replace("{w}", ARTWORK_SIZE)
            .replace("{h}", ARTWORK_SIZE)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response<R> {
    pub data: Vec<R>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResponse<> {
    pub results: SearchResults,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResults<> {
    pub albums: Option<Relationship<Album>>,
    pub songs: Option<Relationship<Song>>,
    pub music_videos: Option<Relationship<MusicVideo>>,
    pub playlists: Option<Relationship<Playlist>>,
    pub artists: Option<Relationship<Artist>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Relationship<R> {
    pub data: Vec<R>,
    pub href: String,
    pub next: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resource<A, R> {
    pub id:            String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub href:          String,
    pub attributes:    A,
    pub relationships: Option<R>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartialResource {
    pub id:            String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub href:          String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayParameters {
    pub id:   String,
    pub kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditorialNotes {
    pub standard: Option<String>,
    pub short:    Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Preview {
    pub url:     String,
    pub artwork: Option<Artwork>,
}

pub type Album = Resource<AlbumAttributes, AlbumRelations>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlbumAttributes {
    pub artist_name:     String,
    pub artwork:         Artwork,
    pub content_rating:  Option<String>,
    pub copyright:       String,
    pub editorial_notes: Option<EditorialNotes>,
    pub genre_names:     Vec<String>,
    pub is_complete:     bool,
    pub is_single:       bool,
    pub name:            String,
    pub record_label:    String,
    pub release_date:    String,
    pub play_params:     Option<PlayParameters>,
    pub track_count:     i32,
    pub url:             String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlbumRelations {
    pub artists: Relationship<Artist>,
    pub genres:  Option<Resource<Genre, NoRelations>>,
    pub tracks:  Relationship<Track>,
}

pub type Artist = Resource<ArtistAttributes, ArtistRelations>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArtistAttributes {
    pub genre_names:     Vec<String>,
    pub editorial_notes: Option<EditorialNotes>,
    pub name:            String,
    pub url:             String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArtistRelations {
    pub albums:      Relationship<Album>,
    pub genres:      Option<Resource<Genre, NoRelations>>,
    pub playlists:   Option<Relationship<Playlist>>,
    pub music_videos: Option<Relationship<MusicVideo>>,
}

pub type Curator = Resource<CuratorAttributes, CuratorRelations>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CuratorAttributes {
    pub artwork:         Artwork,
    pub editorial_notes: EditorialNotes,
    pub name:            String,
    pub url:             String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CuratorRelations {
    pub playlists: Option<Relationship<Playlist>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Track {
    Song(Song),
    MusicVideo(MusicVideo),
}

pub type Song = Resource<SongAttributes, SongRelations>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SongAttributes {
    pub artist_name:        String,
    pub artwork:            Artwork,
    pub composer_name:      Option<String>,
    pub content_rating:     Option<String>,
    pub disc_number:        i32,
    pub duration_in_millis: Option<i32>,
    pub editorial_notes:    Option<EditorialNotes>,
    pub genre_names:        Vec<String>,
    pub isrc:               String,
    pub movement_count:     Option<i32>,
    pub movement_name:      Option<String>,
    pub movement_number:    Option<i32>,
    pub name:               String,
    pub play_params:        Option<PlayParameters>,
    pub previews:           Vec<Preview>,
    pub release_date:       String,
    pub track_number:       i32,
    pub url:                String,
    pub work_name:          Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SongRelations {
    pub albums:  Relationship<PartialResource>,
    pub artists: Relationship<Artist>,
    pub genres:  Option<Resource<Genre, NoRelations>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum PlaylistType {
    UserShared,
    Editorial,
    External,
    PersonalMix
}

pub type Playlist = Resource<PlaylistAttributes, PlaylistRelations>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistAttributes {
    pub artwork:            Option<Artwork>,
    pub curator_name:       Option<String>,
    pub description:        Option<EditorialNotes>,
    pub last_modified_date: String,
    pub name:               String,
    pub playlist_type:      PlaylistType,
    pub play_params:        Option<PlayParameters>,
    pub url:                String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaylistRelations {
    pub curator: Relationship<Curator>,
    pub tracks:  Relationship<Track>,
}


pub type MusicVideo = Resource<MusicVideoAttributes, MusicVideoRelations>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MusicVideoAttributes {
    pub artist_name:        String,
    pub artwork:            Artwork,
    pub content_rating:     Option<String>,
    pub duration_in_millis: Option<i32>,
    pub editorial_notes:    Option<EditorialNotes>,
    pub genre_names:        Vec<String>,
    pub isrc:               String,
    pub name:               String,
    pub play_params:        Option<PlayParameters>,
    pub previews:           Vec<Preview>,
    pub release_date:       String,
    pub track_number:       Option<i32>,
    pub url:                String,
    pub video_sub_type:     Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MusicVideoRelations {
    pub albums:  Resource<Album, NoRelations>,
    pub artists: Resource<Artist, NoRelations>,
    pub genre:   Resource<Genre, NoRelations>,
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

fn fetch(path: &str) -> serde_json::Result<String> {
    let token  = DEVELOPER_TOKEN.to_string();
    let url    = format!("{}{}", BASE_URL, path);
    let mut headers = Headers::new();
    headers.set(Authorization(Bearer { token: token }));
    headers.set(Connection::close());
    let mut res = http::client().get(&url)
                                .headers(headers)
                                .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    Ok(body)
}

pub fn fetch_song(country: &str, id: &str) -> serde_json::Result<Song> {
    let params = "include=artists";
    let path = format!("/catalog/{}/songs/{}?{}", country, id, params);
    let result: serde_json::Result<Response<Song>> = fetch(&path).and_then(|s| serde_json::from_str(&s));
    result.map(|r| r.data.first().unwrap().clone())
}

pub fn fetch_songs(country: &str, ids: Vec<String>) -> serde_json::Result<Vec<Song>> {
    let params = "include=artists";
    let path = format!("/catalog/{}/songs?ids={}&{}", country, ids.join(","), params);
    let result: serde_json::Result<Response<Song>> = fetch(&path).and_then(|s| serde_json::from_str(&s));
    result.map(|r| r.data)
}

pub fn fetch_album(country: &str, id: &str) -> serde_json::Result<Album> {
    let params = "include=artists";
    let path = format!("/catalog/{}/albums/{}?{}", country, id, params);
    let result: serde_json::Result<Response<Album>> = fetch(&path).and_then(|s| serde_json::from_str(&s));
    result.map(|r| r.data.first().unwrap().clone())
}

pub fn fetch_albums(country: &str, ids: Vec<String>) -> serde_json::Result<Vec<Album>> {
    let params = "include=artists";
    let path = format!("/catalog/{}/albums?ids={}&{}", country, ids.join(","), params);
    let result: serde_json::Result<Response<Album>> = fetch(&path).and_then(|s| serde_json::from_str(&s));
    result.map(|r| r.data)
}

pub fn fetch_playlist(country: &str, id: &str) -> serde_json::Result<Playlist> {
    let params = "include=tracks";
    let path = format!("/catalog/{}/playlists/{}?{}", country, id, params);
    let result: serde_json::Result<Response<Playlist>> = fetch(&path).and_then(|s| serde_json::from_str(&s));
    result.map(|r| r.data.first().unwrap().clone())
}

pub fn fetch_artist(country: &str, id: &str) -> serde_json::Result<Artist> {
    let params = "include=albums";
    let path = format!("/catalog/{}/artists/{}?{}", country, id, params);
    let result: serde_json::Result<Response<Artist>> = fetch(&path).and_then(|s| serde_json::from_str(&s));
    result.map(|r| r.data.first().unwrap().clone())
}

pub fn fetch_artists(country: &str, ids: Vec<String>) -> serde_json::Result<Vec<Artist>> {
    let params = "include=albums";
    let path = format!("/catalog/{}/artists?ids={}&{}", country, ids.join(","), params);
    let result: serde_json::Result<Response<Artist>> = fetch(&path).and_then(|s| serde_json::from_str(&s));
    result.map(|r| r.data)
}

pub fn search_artists(country: &str, term: &str) -> serde_json::Result<Vec<Artist>> {
    search(country, term, None, None, Some(vec!["artists"])).map(|res| {
        res.results.artists.map(|a| a.data).unwrap_or(vec![])
    })
}

pub fn search(country: &str, term: &str, limit: Option<i32>, offset: Option<i32>, types: Option<Vec<&str>>) -> serde_json::Result<SearchResponse> {
    let mut params = format!("term={}", term.replace(" ", "+"));
    if let Some(limit) = limit {
        params += &format!("&limit={}", limit);
    }
    if let Some(offset) = offset {
        params += &format!("&offset={}", offset);
    }
    if let Some(types) = types {
        params += &format!("&types={}", types.join(","));
    }
    let path = format!("/catalog/{}/search?{}", &country, &params);
    fetch(&path).and_then(|s| serde_json::from_str(&s))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_fetch_playlist() {
        let playlist = fetch_playlist("jp", "pl.2ff0e502db0c44a598a7cb2261a5e6b2").unwrap();
        assert_eq!(playlist.id, "pl.2ff0e502db0c44a598a7cb2261a5e6b2");
        assert_eq!(playlist.attributes.name, "LILI LIMIT が選ぶ マイプレイリスト");
        assert_eq!(playlist.attributes.curator_name, Some("Apple Music".to_string()));
        assert!(playlist.attributes.artwork.is_some());
        assert_ne!(playlist.attributes.url, "");
    }
    #[test]
    fn test_fetch_album() {
        let album = fetch_album("jp", "1160715126").unwrap();
        assert_eq!(album.id, "1160715126");
        assert_eq!(album.attributes.name, "a.k.a");
        assert_eq!(album.attributes.artist_name, "LILI LIMIT");
        assert_ne!(album.attributes.artwork.url, "");
        assert_ne!(album.attributes.url, "");
    }
    #[test]
    fn test_fetch_song() {
        let song = fetch_song("jp", "1160715431").unwrap();
        assert_eq!(song.id, "1160715431");
        assert_eq!(song.attributes.name, "A Short Film");
        assert_eq!(song.attributes.artist_name, "LILI LIMIT");
        assert_ne!(song.attributes.artwork.url, "");
        assert_ne!(song.attributes.previews[0].url, "");
        assert_ne!(song.attributes.url, "");
    }
}
