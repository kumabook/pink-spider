use std::io::Read;
use std::collections::BTreeMap;
use std::path::Path;
use std::cell::Cell;
use std::rc::Rc;
use html5ever::rcdom::Node;
use html5ever::rcdom::NodeData::Element;
use html5ever::rcdom::{RcDom, Handle};
use html5ever::{parse_document, serialize, Attribute};
use html5ever::tendril::stream::TendrilSink;
use std::default::Default;
use regex::Regex;
use hyper::header::Connection;
use hyper::header::ConnectionOption;
use hyper::header::UserAgent;
use http;
use url::Url;

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
use dom;
use readability;
use readability::Candidate;

use url::percent_encoding::{percent_decode};
use get_env;

lazy_static! {
    static ref USER_AGENT: String = {
        get_env::var("USER_AGENT").unwrap_or("".to_string())
    };
}

const EXPAND_YOUTUBE_PLAYLIST:    bool = true;
const EXPAND_SOUNDCLOUD_PLAYLIST: bool = true;

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
        let url = Url::parse(url)?;
        extract(&mut res, &url)
    } else {
        println!("Failed to get entry html {}: {}", res.status, url);
        Err(Error::NotFound)
    }
}

pub fn extract<R>(input: &mut R, url: &Url) -> Result<ScraperProduct, Error> where R: Read {
    let mut dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(input)
        .unwrap();
    let mut candidates = BTreeMap::new();
    let mut nodes      = BTreeMap::new();
    let mut playlists  = Vec::new();
    let mut tracks     = Vec::new();
    let mut albums     = Vec::new();
    let mut og_props   = Vec::new();
    let handle = dom.document.clone();
    readability::preprocess(&mut dom, handle.clone());
    walk(&mut dom,
         Path::new("/"),
         handle.clone(),
         &mut candidates,
         &mut nodes,
         &mut playlists,
         &mut albums,
         &mut tracks,
         &mut og_props);
    let mut id: &str = "/";
    let mut top_candidate: &Candidate = &Candidate {
        node:  handle.clone(),
        score: Cell::new(0.0),
    };
    for (i, c) in candidates.iter() {
        let score = c.score.get() * (1.0 - readability::get_link_density(c.node.clone()));
        c.score.set(score);
        if score <= top_candidate.score.get() {
            continue;
        }
        id            = i;
        top_candidate = c;
    }
    let mut bytes = vec![];

    let node = top_candidate.node.clone();
    readability::clean(&mut dom, Path::new(id), node.clone(), url, &candidates);

    serialize(&mut bytes, &node, Default::default()).ok();
    let content = String::from_utf8(bytes).unwrap_or("".to_string());

    let mut text: String = String::new();
    dom::extract_text(node.clone(), &mut text, true);
    let og_obj = if og_props.len() > 0 {
        Some(opengraph::Object::new(&og_props))
    } else {
        None
    };
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

fn walk(mut dom:    &mut RcDom,
        id:         &Path,
        handle:     Handle,
        candidates: &mut BTreeMap<String, Candidate>,
        nodes:      &mut BTreeMap<String, Rc<Node>>,
        playlists:  &mut Vec<Playlist>,
        albums:     &mut Vec<Album>,
        tracks:     &mut Vec<Track>,
        og_props:   &mut Vec<(String, String)>) {

    let tag_name: &str = &dom::get_tag_name(handle.clone()).unwrap_or("".to_string());
    match handle.data {
        Element { name: _, ref attrs, .. } => {
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
        }
        _ => (),
    }

    if let Some(id) = id.to_str().map(|id| id.to_string()) {
        nodes.insert(id, handle.clone());
    }

    if readability::is_candidate(handle.clone()) {
        let score = readability::calc_content_score(handle.clone());
        if let Some(c) = id.parent()
            .and_then(|pid| find_or_create_candidate(pid, candidates, nodes))
        {
            c.score.set(c.score.get() + score)
        }
        if let Some(c) = id.parent()
            .and_then(|pid| pid.parent())
            .and_then(|gpid| find_or_create_candidate(gpid, candidates, nodes))
        {
            c.score.set(c.score.get() + score / 2.0)
        }
    }

    for (i, child) in handle.children.borrow().iter().enumerate() {
        walk(&mut dom,
             id.join(i.to_string()).as_path(),
             child.clone(),
             candidates,
             nodes,
             playlists,
             albums,
             tracks,
             og_props)
    }
}

fn find_or_create_candidate<'a>(id: &Path,
                                candidates: &'a mut BTreeMap<String, Candidate>,
                                nodes: &BTreeMap<String, Rc<Node>>) -> Option<&'a Candidate> {
    if let Some(id) = id.to_str().map(|id| id.to_string()) {
        if let Some(node) = nodes.get(&id) {
            if candidates.get(&id).is_none() {
                candidates.insert(id.clone(), Candidate {
                    node:  node.clone(),
                    score: Cell::new(readability::init_content_score(node.clone())),
                });
            }
            return candidates.get(&id)
        }
    }
    None
}


pub fn extract_enclosures_from_tag(tag_name: &str,
                                   attrs: &Vec<Attribute>) -> (Vec<Playlist>, Vec<Album>, Vec<Track>) {
    if tag_name == "iframe" {
        match dom::attr("src", attrs).or(dom::attr("data-src", attrs)) {
            Some(ref src) => extract_enclosures_from_url(src.to_string()),
            None => (vec![], vec![], vec![])
        }
    } else if tag_name == "a" || tag_name == "link" {
        match dom::attr("href", attrs) {
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
    dom::attr(attr_name, attrs)
        .and_then(|property|
                  if property.starts_with("og:") {
                      let end = property.chars().count();
                      let key = unsafe {
                          property.slice_unchecked(3, end)
                      }.to_string();
                      dom::attr("content", attrs).map(|content| (key, content))
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
        assert_eq!(playlists.len(), 2);
        let youtube_tracks: Vec<&Track> = tracks.iter().filter(|&x| x.provider == Provider::YouTube).collect();
        let spotify_tracks: Vec<&Track> = tracks.iter().filter(|&x| x.provider == Provider::Spotify).collect();
        assert!(youtube_tracks.len() > 0);
        assert_eq!(spotify_tracks.len(), 0);
        let spotify_playlists: Vec<&Playlist> = playlists.iter().filter(|&x| x.provider == Provider::Spotify).collect();
        assert_eq!(spotify_playlists[0].tracks.len(), 30);
    }
}
