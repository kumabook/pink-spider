use std::io::Read;
use html5ever::rcdom::NodeData::{
    Document,
    Doctype,
    Text,
    Comment,
    Element,
    ProcessingInstruction
};
use html5ever::rcdom::{RcDom, Handle};
use html5ever::{parse_document, serialize, Attribute};
use html5ever::tendril::stream::TendrilSink;
use html5ever::tree_builder::TreeSink;
use std::default::Default;
use regex::Regex;
use hyper::header::Connection;
use hyper::header::ConnectionOption;
use hyper::header::UserAgent;
use http;

use Provider;
use Track;
use Playlist;
use Album;
use opengraph;
use apple_music;
use youtube;
use soundcloud;
use spotify;
use error::Error;
use model::Enclosure;

use url::percent_encoding::{percent_decode};
use get_env;

lazy_static! {
    static ref USER_AGENT: String = {
        get_env::var("USER_AGENT").unwrap_or("".to_string())
    };
}

const EXPAND_YOUTUBE_PLAYLIST:    bool = true;
const EXPAND_SOUNDCLOUD_PLAYLIST: bool = true;

const THRESHOLD_SCORE: f32 = 100.0;
const PUNCTUATION_WEIGHT: f32 = 10.0;
pub static PUNCTUATIONS: &'static str = r"([、。，．！？]|\.[^A-Za-z0-9]|,[^0-9]|!|\?)";

#[derive(Debug)]
pub struct ScraperProduct {
    pub content:   String,
    pub text:      String,
    pub playlists: Vec<Playlist>,
    pub albums:    Vec<Album>,
    pub tracks:    Vec<Track>,
    pub og_obj:    Option<opengraph::Object>,
}

pub fn scrape(url: &str) -> Result<ScraperProduct, Error> {
    let client      = http::client();
    let mut builder = client.get(url)
        .header(Connection(vec![ConnectionOption::Close]));
    if *USER_AGENT != "" {
        builder = builder.header(UserAgent(USER_AGENT.to_string()));
    }
    let mut res = try!(builder.send());
    if res.status.is_success() {
        extract(&mut res)
    } else {
        println!("Failed to get entry html {}: {}", res.status, url);
        Err(Error::NotFound)
    }
}

pub fn extract<R>(input: &mut R) -> Result<ScraperProduct, Error> where R: Read {
    let mut dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(input)
        .unwrap();
    let mut playlists = Vec::new();
    let mut tracks    = Vec::new();
    let mut albums    = Vec::new();
    let mut og_props  = Vec::new();
    let handle = dom.document.clone();
    walk(&mut dom,
         handle,
         &mut playlists,
         &mut albums,
         &mut tracks,
         &mut og_props);
    let mut bytes = vec![];
    serialize(&mut bytes, &dom.document, Default::default()).ok();
    let content = String::from_utf8(bytes).unwrap_or("".to_string());
    let og_obj = if og_props.len() > 0 {
        Some(opengraph::Object::new(&og_props))
    } else {
        None
    };
    let mut text: String = String::new();
    extract_text(dom.document, &mut text);
    Ok(ScraperProduct {
        content:   content,
        text:      text,
        playlists: playlists,
        albums:    albums,
        tracks:    tracks,
        og_obj:    og_obj
    })
}

pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

// This is not proper HTML serialization, of course.
fn walk(mut dom:   &mut RcDom,
        handle:    Handle,
        playlists: &mut Vec<Playlist>,
        albums:    &mut Vec<Album>,
        tracks:    &mut Vec<Track>,
        og_props:  &mut Vec<(String, String)>) -> bool {
    let mut useless = false;
    match handle.data {
        Document       => (),
        Doctype { .. } => (),
        Text { ref contents } => {
            let s = contents.borrow();
            if s.trim().len() == 0 {
                useless = true
            }
        },
        Comment { .. } => useless = true,
        Element { ref name, ref attrs, .. } => {
            let tag_name = name.local.as_ref();
            let mut ps = extract_opengraph_metadata_from_tag(tag_name, &attrs.borrow());
            og_props.append(&mut ps);
            let (ps, als, ts) = extract_enclosures_from_tag(tag_name, &attrs.borrow());
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
            match tag_name.to_lowercase().as_ref() {
                "script"   => useless = true,
                "link"     => useless = true,
                "style"    => useless = true,
                "noscript" => useless = true,
                "meta"     => useless = true,
                "div" | "center" | "td" => {
                    let score = calc_content_score(handle.clone());
                    if THRESHOLD_SCORE > score {
                        useless = true
                    }
                },
                "ul" | "dl" | "ol" => {
                    if is_link_list(handle.clone()) {
                        useless = true
                    }
                },
                "footer" => useless = true,
                "header" => useless = true,
                _        => (),
            }
            clean_attr(&mut *attrs.borrow_mut(), "class");
            clean_attr(&mut *attrs.borrow_mut(), "style");
        },
        ProcessingInstruction { .. } => unreachable!()
    }
    let mut useless_nodes = vec![];
    for child in handle.children.borrow().iter() {
        if walk(&mut dom, child.clone(), playlists, albums, tracks, og_props) {
            useless_nodes.push(child.clone());
        }
    }
    for node in useless_nodes.iter() {
        dom.remove_from_parent(node);
    }
    if is_empty(handle) {
        useless = true
    }
    useless
}

fn clean_attr(attrs: &mut Vec<Attribute>, attr_name: &str) {
    if let Some(index) = attrs.iter().position(|attr| {
        let name = attr.name.local.as_ref();
        name == attr_name
    }) {
        attrs.remove(index);
    }
}

fn calc_content_score(handle: Handle) -> f32 {
    let mut score: f32 = 0.0;
    for child in handle.children.borrow().iter() {
        let c = child.clone();
        match c.data {
            Text { ref contents } => {
                let re = Regex::new(PUNCTUATIONS).unwrap();
                let s = contents.borrow();
                let mat = re.find_iter(&s.trim());
                score += mat.count() as f32 * PUNCTUATION_WEIGHT;
            },
            Element { .. } => {
                score += calc_content_score(child.clone());
            },
            _ => ()
        }
    }
    return score
}

fn is_empty(handle: Handle) -> bool {
    for child in handle.children.borrow().iter() {
        let c = child.clone();
        match c.data {
            Text { ref contents } => {
                if contents.borrow().trim().len() > 0 {
                    return false
                }
            },
            Element { ref name, .. } => {
                let tag_name = name.local.as_ref();
                match tag_name.to_lowercase().as_ref() {
                    "li" | "dt" | "dd" | "p" | "div" => {
                        if !is_empty(child.clone()) {
                            return false
                        }
                    },
                    _ => return false,
                }
            },
            _ => ()
        }
    }
    match handle.data {
        Element { ref name, .. } => {
            let tag_name = name.local.as_ref();
            match tag_name.to_lowercase().as_ref() {
                "li" | "dt" | "dd" | "p" | "div" | "canvas" => {
                    true
                },
                _ => false,
            }
        },
        _ => false,
    }
}

fn is_link_list(handle: Handle) -> bool {
    let rate = evaluate_list(handle);
    rate > 7.5
}

fn evaluate_list(handle: Handle) -> f32 {
    let mut hit: i32 = 0;
    let mut len: i32 = 0;
    for child in handle.children.borrow().iter() {
        let c = child.clone();
        match c.data {
            Element { ref name, .. } => {
                let tag_name = name.local.as_ref();
                match tag_name.to_lowercase().as_ref() {
                    "li" | "dt" | "dd" => {
                        len += 1;
                        if has_link(child.clone()) {
                            hit += 1
                        }
                    }
                    _ => (),
                }
            },
            _ => ()
        }
    }
    if len == 0 {
        0.0
    } else {
        9.0 * ((hit / len) as f32).powi(2) + 1.0
    }
}

fn has_link(handle: Handle) -> bool {
    match handle.data {
        Element { ref name, .. } => {
            let tag_name = name.local.as_ref();
            match tag_name.to_lowercase().as_ref() {
                "a" => return true,
                _ => (),
            }
        }
        _ => (),
    }
    for child in handle.children.borrow().iter() {
        if has_link(child.clone()) {
            return true
        }
    }
    return false
}

fn extract_text(handle: Handle, text: &mut String) {
    for child in handle.children.borrow().iter() {
        let c = child.clone();
        match c.data {
            Text { ref contents } => {
                text.push_str(contents.borrow().trim());
            },
            Element { .. } => {
                extract_text(child.clone(), text);
            },
            _ => ()
        }
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
        match attr("src", attrs).or(attr("data-src", attrs)) {
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

pub fn extract_opengraph_metadata_from_tag(tag_name: &str,
                                           attrs: &Vec<Attribute>) -> Vec<(String, String)> {

    let mut og_props = vec!();
    if tag_name == "meta" {
        match extract_opengraph_prop("property", attrs) {
            Some((key, content)) => og_props.push((key, content)),
            None                 => (),
        }
        match extract_opengraph_prop("name", attrs) {
            Some((key, content)) => og_props.push((key, content)),
            None                 => (),
        }
    }
    og_props
}

fn extract_opengraph_prop<'a>(attr_name: &str, attrs: &Vec<Attribute>) -> Option<(String, String)> {
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

fn create_spotify_playlist(user_id: String, playlist_id: String) -> (Vec<Playlist>, Vec<Album>, Vec<Track>) {
    let mut playlist = Playlist::new(Provider::Spotify, playlist_id);
    let _ = playlist.set_owner_id(Some(user_id)).fetch_props();
    (vec![playlist], vec![], vec![])
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

    match extract_identifier(&decoded, apple_music::SONG_URL) {
        Some(identifier) => {
            let country = apple_music::country(&url);
            if let Ok(song) = apple_music::fetch_song(&identifier, &country) {
                return (vec![], vec![], vec![Track::from_am_song(&song)])
            };
        },
        None => ()
    }
    match extract_identifier(&decoded, apple_music::ALBUM_URL) {
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
    match extract_identifier(&decoded, apple_music::PLAYLIST_URL) {
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
    match extract_identifier(&decoded, youtube::WATCH) {
        Some(identifier) => return (vec![], vec![], vec![Track::new(Provider::YouTube, identifier)]),
        None             => ()
    }
    match extract_identifier(&decoded, youtube::LIST) {
        Some(identifier) => return fetch_youtube_playlist(&identifier),
        None             => ()
    }
    match extract_identifier(&decoded, youtube::EMBED) {
        Some(identifier) => return (vec![], vec![], vec![Track::new(Provider::YouTube, identifier)]),
        None             => ()
    }
    match extract_identifier(&decoded, soundcloud::TRACK) {
        Some(identifier) => return (vec![], vec![], vec![Track::new(Provider::SoundCloud, identifier)]),
        None             => ()
    }
    match extract_identifier(&decoded, soundcloud::PLAYLIST) {
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
    match extract_identifier(&decoded, soundcloud::USER) {
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
    match extract_identifier(&decoded, spotify::TRACK_URI) {
        Some(identifier) => return (vec![], vec![], fetch_spotify_track(identifier)),
        None             => ()
    }
    match extract_identifier(&decoded, spotify::TRACK_OPEN) {
        Some(identifier) => return (vec![], vec![], fetch_spotify_track(identifier)),
        None             => ()
    }
    match extract_identifier(&decoded, spotify::TRACK_EMBED) {
        Some(identifier) => return (vec![], vec![], fetch_spotify_track(identifier)),
        None             => ()
    }
    match extract_identifier(&decoded, spotify::PLAYLIST_URI).and_then(
        |uri| spotify::parse_uri_as_playlist(&uri)) {
        Some((uid, pid)) => return create_spotify_playlist(uid, pid),
        None => (),
    }
    match extract_identifier(&decoded, spotify::PLAYLIST_OPEN).and_then(
        |url| spotify::parse_open_url_as_playlist(&url)) {
        Some((uid, pid)) => return create_spotify_playlist(uid, pid),
        None => ()
    }
    match extract_identifier(&decoded, spotify::PLAYLIST_EMBED).and_then(
        |url| spotify::parse_embed_url_as_playlist(&url)) {
        Some((uid, pid)) => return create_spotify_playlist(uid, pid),
        None => ()
    }
    match extract_identifier(&decoded, spotify::ALBUM_URI) {
        Some(identifier) => return fetch_spotify_album(identifier),
        None             => ()
    }
    match extract_identifier(&decoded, spotify::ALBUM_OPEN) {
        Some(identifier) => return fetch_spotify_album(identifier),
        None             => ()
    }
    match extract_identifier(&decoded, spotify::ALBUM_EMBED) {
        Some(identifier) => return fetch_spotify_album(identifier),
        None             => ()
    }
    return (vec![], vec![], vec![])
}

#[cfg(test)]
mod test {
    use super::scrape;
    use super::extract_identifier;
    use youtube;
    use soundcloud;
    use Provider;
    use Track;
    use Playlist;

    #[test]
    fn test_extract_identifier() {
        let soundcloud_src = "https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/195425494/stream&auto_play=false&hide_related=false&show_comments=true&show_user=true&show_reposts=false&visual=true";
        match extract_identifier(soundcloud_src, soundcloud::TRACK) {
            Some(identifier) => assert_eq!(identifier, "195425494".to_string()),
            None             => assert!(false)
        }
        let youtube_embed = "https://www.youtube.com/embed/X8tOngmlES0?rel=0";
        match extract_identifier(youtube_embed, youtube::EMBED) {
            Some(identifier) => assert_eq!(identifier, "X8tOngmlES0".to_string()),
            None             => assert!(false)
        }

        let youtube_watch = "https://www.youtube.com/watch?v=oDuif301F-8";
        match extract_identifier(youtube_watch, youtube::WATCH) {
            Some(identifier) => assert_eq!(identifier, "oDuif301F-8".to_string()),
            None             => assert!(false)
        }

        let youtube_list = "https://www.youtube.com/embed/videoseries?list=PLy8LZ8FM-o0ViuGAF68RAaXkQ8V-3dbTX";
        match extract_identifier(youtube_list, youtube::LIST) {
            Some(identifier) => assert_eq!(identifier, "PLy8LZ8FM-o0ViuGAF68RAaXkQ8V-3dbTX".to_string()),
            None             => assert!(false)
        }
    }
    #[test]
    fn test_scrape() {
        let url       = "http://spincoaster.com/spincoaster-breakout-2017";
        let product   = scrape(url).unwrap();
        let playlists = product.playlists;
        let tracks    = product.tracks;
        assert_eq!(playlists.len(), 3);
        let youtube_tracks: Vec<&Track> = tracks.iter().filter(|&x| x.provider == Provider::YouTube).collect();
        let spotify_tracks: Vec<&Track> = tracks.iter().filter(|&x| x.provider == Provider::Spotify).collect();
        assert_eq!(youtube_tracks.len(), 15);
        assert_eq!(spotify_tracks.len(), 0);
        let spotify_playlists: Vec<&Playlist> = playlists.iter().filter(|&x| x.provider == Provider::Spotify).collect();
        assert_eq!(spotify_playlists[0].tracks.len(), 30);
    }
}
