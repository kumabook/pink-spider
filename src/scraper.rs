use html5ever::rcdom::{Document, Doctype, Text, Comment, Element};
use html5ever::rcdom::{RcDom, Handle};
use html5ever::{parse_document, Attribute};
use tendril::stream::TendrilSink;
use std::default::Default;
use regex::Regex;
use hyper::Client;
use hyper::header::Connection;
use hyper::header::ConnectionOption;
use hyper::header::UserAgent;

use Provider;
use Track;
use Playlist;
use Album;
use open_graph;
use apple_music;
use youtube;
use soundcloud;
use spotify;
use error::Error;
use model::Enclosure;

use url::percent_encoding::{percent_decode};
use get_env;

static APPLE_MUSIC_SONG:      &'static str = r"tools.applemusic.com/embed/v1/song/([a-zA-Z0-9_-]+)";
static APPLE_MUSIC_ALBUM:     &'static str = r"tools.applemusic.com/embed/v1/album/([a-zA-Z0-9_-]+)";
static APPLE_MUSIC_PLAYLIST:  &'static str = r"tools.applemusic.com/embed/v1/playlist/pl.([a-zA-Z0-9_-]+)";
static YOUTUBE_EMBED:         &'static str = r"www.youtube.com/embed/([a-zA-Z0-9_-].+)";
static YOUTUBE_LIST:          &'static str = r"www.youtube.com/embed/videoseries\?list=([a-zA-Z0-9_-]+)";
static YOUTUBE_WATCH:         &'static str = r"www.youtube.com/watch\?v=([a-zA-Z0-9_-]+)";
static SOUNDCLOUD_TRACK:      &'static str = r"api.soundcloud.com/tracks/([a-zA-Z0-9_-]+)";
static SOUNDCLOUD_PLAYLIST:   &'static str = r"api.soundcloud.com/playlists/([a-zA-Z0-9_-]+)";
static SOUNDCLOUD_USER:       &'static str = r"api.soundcloud.com/users/([a-zA-Z0-9_-]+)";
static SPOTIFY_TRACK_OPEN:    &'static str = r"open.spotify.com/track/([a-zA-Z0-9_-]+)";
static SPOTIFY_TRACK:         &'static str = r"spotify:track:([a-zA-Z0-9_-]+)";
static SPOTIFY_PLAYLIST_OPEN: &'static str = r"(open.spotify.com/user/([a-zA-Z0-9_-]+)/playlist/([a-zA-Z0-9_-]+))";
static SPOTIFY_PLAYLIST:      &'static str = r"(spotify:user:([a-zA-Z0-9_-]+):playlist:([a-zA-Z0-9_-]+))";
static SPOTIFY_ALBUM_OPEN:    &'static str = r"open.spotify.com/album/([a-zA-Z0-9_-]+)";
static SPOTIFY_ALBUM:         &'static str = r"spotify:album:([a-zA-Z0-9_-]+)";

lazy_static! {
    static ref USER_AGENT: String = {
        get_env::var("USER_AGENT").unwrap_or("".to_string())
    };
}

const EXPAND_YOUTUBE_PLAYLIST:    bool = true;
const EXPAND_SOUNDCLOUD_PLAYLIST: bool = true;

#[derive(Debug)]
pub struct ScraperProduct {
    pub playlists: Vec<Playlist>,
    pub albums:    Vec<Album>,
    pub tracks:    Vec<Track>,
    pub og_obj:    Option<open_graph::Object>,
}

pub fn extract(url: &str) -> Result<ScraperProduct, Error> {
    let client = Client::new();
    let mut builder = client.get(url)
        .header(Connection(vec![ConnectionOption::Close]));
    if *USER_AGENT != "" {
        builder = builder.header(UserAgent(USER_AGENT.to_string()));
    }
    let mut res = try!(builder.send());
    if res.status.is_success() {
        let dom = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut res)
            .unwrap();
        let mut playlists = Vec::new();
        let mut tracks    = Vec::new();
        let mut albums    = Vec::new();
        let mut og_props  = Vec::new();
        walk(dom.document, &mut playlists, &mut albums, &mut tracks, &mut og_props);
        let og_obj = if og_props.len() > 0 {
            Some(open_graph::Object::new(&og_props))
        } else {
            None
        };
        Ok(ScraperProduct {
            playlists: playlists,
            albums:    albums,
            tracks:    tracks,
            og_obj:    og_obj
        })
    } else {
        println!("Failed to get entry html {}: {}", res.status, url);
        Err(Error::NotFound)
    }
}

// This is not proper HTML serialization, of course.
fn walk(handle:    Handle,
        playlists: &mut Vec<Playlist>,
        albums:    &mut Vec<Album>,
        tracks:    &mut Vec<Track>,
        og_props:  &mut Vec<(String, String)>) {
    let node = handle.borrow();
    match node.node {
        Document         => (),
        Doctype(_, _, _) => (),
        Text(_)          => (),
        Comment(_)       => (),
        Element(ref name, _, ref attrs) => {
            let tag_name = name.local.as_ref();
            let mut ps = extract_open_graph_metadata_from_tag(tag_name, attrs);
            og_props.append(&mut ps);
            let (ps, als, ts) = extract_enclosures_from_tag(tag_name, attrs);
            for playlist in ps.iter().cloned() {
                if !(playlists).iter().any(|p| playlist == *p) {
                    (*playlists).push(playlist)
                }
            }
            for album in als.iter().cloned() {
                if !(albums).iter().any(|a| album == *a) {
                    (*albums).push(album)
                }
            }
            for track in ts.iter().cloned() {
                if !(tracks).iter().any(|t| track == *t) {
                    (*tracks).push(track)
                }
            }
        }
    }
    for child in node.children.iter() {
        walk(child.clone(), playlists, albums, tracks, og_props);
    }
}

fn attr(attr_name: &str, attrs: &Vec<Attribute>) -> Option<String> {
    for attr in attrs.iter() {
        if attr.name.local.as_ref() == attr_name {
            return Some(attr.value.to_string())
        }
    }
    None
}

pub fn extract_enclosures_from_tag(tag_name: &str,
                                   attrs: &Vec<Attribute>) -> (Vec<Playlist>, Vec<Album>, Vec<Track>) {
    if tag_name == "iframe" {
        match attr("src", attrs) {
            Some(ref src) => extract_enclosures_from_url(src.to_string()),
            None => (vec![], vec![], vec![])
        }
    } else if tag_name == "a" || tag_name == "link" {
        match attr("href", attrs) {
            Some(ref href) => extract_enclosures_from_url(href.to_string()),
            None => (vec![], vec![], vec![])
        }
    } else {
        (vec![], vec![], vec![])
    }
}

pub fn extract_open_graph_metadata_from_tag(tag_name: &str,
                                            attrs: &Vec<Attribute>) -> Vec<(String, String)> {

    let mut og_props = vec!();
    if tag_name == "meta" {
        match extract_open_graph_prop("property", attrs) {
            Some((key, content)) => og_props.push((key, content)),
            None                 => (),
        }
        match extract_open_graph_prop("name", attrs) {
            Some((key, content)) => og_props.push((key, content)),
            None                 => (),
        }
    }
    og_props
}

fn extract_open_graph_prop<'a>(attr_name: &str, attrs: &Vec<Attribute>) -> Option<(String, String)> {
    attr(attr_name, attrs)
        .and_then(|property|
                  if property.starts_with("og:") {
                      let end = property.chars().count();
                      let key = unsafe {
                          property.slice_unchecked(3, end)
                      }.to_string();
                      attr("content", attrs).map(|content| (key, content))
                  } else {
                      None
                  })
}

fn extract_identifier(value: &str, regex_str: &str) -> Option<String> {
    match Regex::new(regex_str) {
        Ok(re) => match re.captures(value) {
            Some(cap) => {
                let strs: Vec<&str> = cap[1].split('?').collect();
                return Some(strs[0].to_string())
            },
            None => None
        },
        Err(_) => None
    }
}

fn fetch_spotify_playlist(uri: &str, regex: &str) -> (Vec<Playlist>, Vec<Album>, Vec<Track>) {
    match Regex::new(regex) {
        Ok(re) => match re.captures(uri) {
            Some(cap) => {
                let user_id     = cap[2].to_string();
                let playlist_id = cap[3].to_string();
                return match spotify::fetch_playlist(&user_id, &playlist_id) {
                    Ok(playlist) => {
                        (vec![Playlist::from_sp_playlist(&playlist)], vec![], vec![])
                    },
                    Err(_)       => (vec![], vec![], vec![])
                }
            },
            None => (vec![], vec![], vec![])
        },
        Err(_) => (vec![], vec![], vec![])
    }
}

fn fetch_spotify_album(identifier: String) -> (Vec<Playlist>, Vec<Album>, Vec<Track>) {
    match spotify::fetch_album(&identifier) {
        Ok(album) => {
            (vec![], vec![Album::from_sp_album(&album)], vec![])
        },
        Err(_) => (vec![], vec![Album::new(Provider::Spotify, identifier)], vec![]),
    }
}

fn fetch_spotify_track(identifier: String) -> Vec<Track> {
    match spotify::fetch_track(&identifier) {
        Ok(t) => {
            let ref mut track = Track::new(Provider::Spotify, identifier);
            track.update_with_sp_track(&t);
            vec![track.clone()]
        },
        Err(_) => vec![Track::new(Provider::Spotify, identifier)],
    }
}

fn fetch_youtube_playlist(id: &str) -> (Vec<Playlist>, Vec<Album>, Vec<Track>) {
    let items = youtube::fetch_playlist_items(id)
        .map(|res| res.items)
        .unwrap_or(vec![]);
    let tracks = if EXPAND_YOUTUBE_PLAYLIST {
        items.iter()
            .map(|ref i| Track::from_yt_playlist_item(i))
            .collect::<Vec<_>>()
    } else {
        vec![]
    };
    let playlists = match youtube::fetch_playlist(id) {
        Ok(res) => res.items.iter()
            .map(|ref i| Playlist::from_yt_playlist(i, &items))
            .collect::<Vec<_>>(),
        Err(_)  => vec![],
    };
    (playlists, vec![], tracks)
}

fn extract_enclosures_from_url(url: String) -> (Vec<Playlist>, Vec<Album>, Vec<Track>) {
    let decoded = percent_decode(url.as_bytes()).decode_utf8_lossy().into_owned();

    match extract_identifier(&decoded, APPLE_MUSIC_SONG) {
        Some(identifier) => {
            let country = apple_music::country(&url);
            if let Ok(song) = apple_music::fetch_song(&identifier, &country) {
                return (vec![], vec![], vec![Track::from_am_song(&song)])
            };
        },
        None => ()
    }
    match extract_identifier(&decoded, APPLE_MUSIC_ALBUM) {
        Some(identifier) => {
            let country = apple_music::country(&url);
            match apple_music::fetch_album(&identifier, &country) {
                Ok(album) => return (vec![], vec![Album::from_am_album(&album)], vec![]),
                Err(_) => (),
            }
            return (vec![], vec![], vec![])
        },
        None => ()
    }
    match extract_identifier(&decoded, APPLE_MUSIC_PLAYLIST) {
        Some(identifier) => {
            let country = apple_music::country(&url);
            match apple_music::fetch_playlist(&identifier, &country) {
                Ok(playlist) => return (vec![Playlist::from_am_playlist(&playlist)], vec![], vec![]),
                Err(_) => (),
            }
            return (vec![], vec![], vec![])
        },
        None => ()
    }
    match apple_music::parse_url_as_album(&decoded) {
        Some((country, _, _, Some(song_id))) => {
            if let Ok(song) = apple_music::fetch_song(&song_id, &country) {
                return (vec![], vec![], vec![Track::from_am_song(&song)])
            };
            return (vec![], vec![], vec![])
        },
        Some((_, _, _, None)) => (),
        None => ()
    }
    match apple_music::parse_url_as_playlist(&decoded) {
        Some((country, _, identifier, _)) => {
            match apple_music::fetch_playlist(&identifier, &country) {
                Ok(playlist) => return (vec![Playlist::from_am_playlist(&playlist)], vec![], vec![]),
                Err(_) => (),
            }
            return (vec![], vec![], vec![])
        },
        None => ()
    }
    match extract_identifier(&decoded, YOUTUBE_WATCH) {
        Some(identifier) => return (vec![], vec![], vec![Track::new(Provider::YouTube, identifier)]),
        None             => ()
    }
    match extract_identifier(&decoded, YOUTUBE_LIST) {
        Some(identifier) => return fetch_youtube_playlist(&identifier),
        None             => ()
    }
    match extract_identifier(&decoded, YOUTUBE_EMBED) {
        Some(identifier) => return (vec![], vec![], vec![Track::new(Provider::YouTube, identifier)]),
        None             => ()
    }
    match extract_identifier(&decoded, SOUNDCLOUD_TRACK) {
        Some(identifier) => return (vec![], vec![], vec![Track::new(Provider::SoundCloud, identifier)]),
        None             => ()
    }
    match extract_identifier(&decoded, SOUNDCLOUD_PLAYLIST) {
        Some(identifier) => return match soundcloud::fetch_playlist(&identifier) {
            Ok(playlist) => {
                let tracks = if EXPAND_SOUNDCLOUD_PLAYLIST {
                    playlist.tracks
                        .iter()
                        .map(|ref t| Track::from_sc_track(t))
                        .collect::<Vec<_>>()
                } else {
                    vec![]
                };
                (vec![Playlist::from_sc_playlist(&playlist)], vec![], tracks)
            },
            Err(_)       => (vec![], vec![], vec![]),
        },
        None => ()
    }
    match extract_identifier(&decoded, SOUNDCLOUD_USER) {
        Some(identifier) => return match soundcloud::fetch_user_tracks(&identifier) {
            Ok(tracks) => {
                let tracks = tracks
                    .iter()
                    .map(|ref t| Track::from_sc_track(t))
                    .collect::<Vec<_>>();
                (vec![], vec![], tracks)
            },
            Err(_)       => (vec![], vec![], vec![]),
        },
        None => ()
    }
    match extract_identifier(&decoded, SPOTIFY_TRACK) {
        Some(identifier) => return (vec![], vec![], fetch_spotify_track(identifier)),
        None             => ()
    }
    match extract_identifier(&decoded, SPOTIFY_TRACK_OPEN) {
        Some(identifier) => return (vec![], vec![], fetch_spotify_track(identifier)),
        None             => ()
    }
    match extract_identifier(&decoded, SPOTIFY_PLAYLIST) {
        Some(uri) => return fetch_spotify_playlist(&uri, SPOTIFY_PLAYLIST),
        None      => ()
    }
    match extract_identifier(&decoded, SPOTIFY_PLAYLIST_OPEN) {
        Some(uri) => return fetch_spotify_playlist(&uri, SPOTIFY_PLAYLIST_OPEN),
        None      => ()
    }
    match extract_identifier(&decoded, SPOTIFY_ALBUM) {
        Some(identifier) => return fetch_spotify_album(identifier),
        None             => ()
    }
    match extract_identifier(&decoded, SPOTIFY_ALBUM_OPEN) {
        Some(identifier) => return fetch_spotify_album(identifier),
        None             => ()
    }
    return (vec![], vec![], vec![])
}

#[cfg(test)]
mod test {
    use super::extract;
    use super::extract_identifier;
    use Provider;
    use Track;
    use Playlist;

    #[test]
    fn test_extract_identifier() {
        let soundcloud_src = "https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/195425494/stream&auto_play=false&hide_related=false&show_comments=true&show_user=true&show_reposts=false&visual=true";
        match extract_identifier(soundcloud_src, super::SOUNDCLOUD_TRACK) {
            Some(identifier) => assert_eq!(identifier, "195425494".to_string()),
            None             => assert!(false)
        }
        let youtube_embed = "https://www.youtube.com/embed/X8tOngmlES0?rel=0";
        match extract_identifier(youtube_embed, super::YOUTUBE_EMBED) {
            Some(identifier) => assert_eq!(identifier, "X8tOngmlES0".to_string()),
            None             => assert!(false)
        }

        let youtube_watch = "https://www.youtube.com/watch?v=oDuif301F-8";
        match extract_identifier(youtube_watch, super::YOUTUBE_WATCH) {
            Some(identifier) => assert_eq!(identifier, "oDuif301F-8".to_string()),
            None             => assert!(false)
        }

        let youtube_list = "https://www.youtube.com/embed/videoseries?list=PLy8LZ8FM-o0ViuGAF68RAaXkQ8V-3dbTX";
        match extract_identifier(youtube_list, super::YOUTUBE_LIST) {
            Some(identifier) => assert_eq!(identifier, "PLy8LZ8FM-o0ViuGAF68RAaXkQ8V-3dbTX".to_string()),
            None             => assert!(false)
        }
    }
    #[test]
    fn test_extract() {
        let url       = "http://spincoaster.com/spincoaster-breakout-2017";
        let product   = extract(url).unwrap();
        let playlists = product.playlists;
        let tracks    = product.tracks;
        assert_eq!(playlists.len(), 2);
        let youtube_tracks: Vec<&Track> = tracks.iter().filter(|&x| x.provider == Provider::YouTube).collect();
        let spotify_tracks: Vec<&Track> = tracks.iter().filter(|&x| x.provider == Provider::Spotify).collect();
        assert_eq!(youtube_tracks.len(), 15);
        assert_eq!(spotify_tracks.len(), 0);
        let spotify_playlists: Vec<&Playlist> = playlists.iter().filter(|&x| x.provider == Provider::Spotify).collect();
        assert_eq!(spotify_playlists[0].tracks.len(), 30);
    }
}
