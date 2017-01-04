extern crate html5ever;
extern crate tendril;
extern crate regex;
extern crate hyper;
extern crate string_cache;
extern crate url;

use html5ever::rcdom::{Document, Doctype, Text, Comment, Element};
use html5ever::rcdom::{RcDom, Handle};
use html5ever::{parse_document, Attribute};
use tendril::stream::TendrilSink;
use std::default::Default;

use regex::Regex;
use hyper::Client;
use hyper::header::Connection;
use hyper::header::ConnectionOption;

use Provider;
use Track;
use open_graph;
use soundcloud;
use youtube;
use error::Error;

use url::percent_encoding::{percent_decode};

static YOUTUBE_EMBED:       &'static str = r"www.youtube.com/embed/([a-zA-Z0-9_-].+)";
static YOUTUBE_LIST:        &'static str = r"www.youtube.com/embed/videoseries\?list=([a-zA-Z0-9_-]+)";
static YOUTUBE_WATCH:       &'static str = r"www.youtube.com/watch\?v=([a-zA-Z0-9_-]+)";
static SOUNDCLOUD_TRACK:    &'static str = r"api.soundcloud.com/tracks/([a-zA-Z0-9_-]+)";
static SOUNDCLOUD_PLAYLIST: &'static str = r"api.soundcloud.com/playlists/([a-zA-Z0-9_-]+)";
static SOUNDCLOUD_USER:     &'static str = r"api.soundcloud.com/users/([a-zA-Z0-9_-]+)";

#[derive(Debug)]
pub struct ScraperProduct {
    pub tracks: Vec<Track>,
    pub og_obj: Option<open_graph::Object>,
}

pub fn extract(url: &str) -> Result<ScraperProduct, Error> {
    let client = Client::new();
    let mut res = client.get(url)
        .header(Connection(vec![ConnectionOption::Close]))
        .send()
        .unwrap();
    if res.status.is_success() {
        let dom = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut res)
            .unwrap();
        let mut tracks   = Vec::new();
        let mut og_props = Vec::new();
        walk(0, dom.document, &mut tracks, &mut og_props);
        let og_obj = if og_props.len() > 0 {
            Some(open_graph::Object::new(&og_props))
        } else {
            None
        };
        Ok(ScraperProduct {
            tracks: tracks,
            og_obj: og_obj
        })
    } else {
        println!("Failed to get entry html {}: {}", res.status, url);
        Err(Error::NotFound)
    }
}

// This is not proper HTML serialization, of course.
fn walk(indent: usize,
        handle: Handle,
        tracks: &mut Vec<Track>,
        og_props: &mut Vec<(String, String)>) {
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
            let ts = extract_tracks_from_tag(tag_name, attrs);
            for track in ts.iter().cloned() {
                if !(tracks).iter().any(|t| track == *t) {
                    (*tracks).push(track)
                }
            }
        }
    }
    for child in node.children.iter() {
        walk(indent+4, child.clone(), tracks, og_props);
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

pub fn extract_tracks_from_tag(tag_name: &str,
                               attrs: &Vec<Attribute>) -> Vec<Track> {
    if tag_name == "iframe" {
        match attr("src", attrs) {
            Some(ref src) => extract_tracks_from_url(src.to_string()),
            None => vec![]
        }
    } else if tag_name == "a" || tag_name == "link" {
        match attr("href", attrs) {
            Some(ref href) => extract_tracks_from_url(href.to_string()),
            None => vec![]
        }
    } else {
        vec![]
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

fn extract_tracks_from_url(url: String) -> Vec<Track> {
    let decoded = percent_decode(url.as_bytes()).decode_utf8_lossy().into_owned();
    match extract_identifier(&decoded, YOUTUBE_WATCH) {
        Some(identifier) => {
            return vec![Track::new(Provider::YouTube, identifier)]
        },
        None => ()
    }
    match extract_identifier(&decoded, YOUTUBE_LIST) {
        Some(identifier) => {
            return match youtube::fetch_playlist(&identifier) {
                Ok(res) => res.items.iter()
                                    .map(|ref i| Track::from_yt_playlist_item(i))
                                    .collect::<Vec<_>>(),
                Err(_)       => vec![]
            }
        },
        None => ()
    }
    match extract_identifier(&decoded, YOUTUBE_EMBED) {
        Some(identifier) => {
            return vec![Track::new(Provider::YouTube, identifier)]
        },
        None => ()
    }
    match extract_identifier(&decoded, SOUNDCLOUD_TRACK) {
        Some(identifier) => {
            return vec![Track::new(Provider::SoundCloud, identifier)]
        },
        None => ()
    }
    match extract_identifier(&decoded, SOUNDCLOUD_PLAYLIST) {
        Some(identifier) => {
            return match soundcloud::fetch_playlist(&identifier) {
                Ok(playlist) => playlist.tracks
                                        .iter()
                                        .map(|ref t| Track::from_sc_track(t))
                                        .collect::<Vec<_>>(),
                Err(_)       => vec![]
            }
        },
        None => ()
    }
    match extract_identifier(&decoded, SOUNDCLOUD_USER) {
        Some(identifier) => {
            print!("user {}", identifier);
            return match soundcloud::fetch_user_tracks(&identifier) {
                Ok(tracks) => tracks.iter()
                                    .map(|ref t| Track::from_sc_track(t))
                                    .collect::<Vec<_>>(),
                Err(_)     => vec![]
            }
        },
        None => ()
    }
    return vec![]
}

#[cfg(test)]
mod test {
    use super::extract_identifier;

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
}
