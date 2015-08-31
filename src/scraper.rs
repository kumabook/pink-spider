extern crate html5ever;
extern crate tendril;
extern crate regex;
extern crate hyper;
extern crate string_cache;

use std::str::FromStr;

use self::tendril::Tendril;

use self::html5ever::rcdom::{Document, Doctype, Text, Comment, Element};
use self::html5ever::rcdom::{RcDom, Handle};
use self::html5ever::{parse, one_input, Attribute};
use std::default::Default;
use std::io::Read;

use self::regex::Regex;
use self::hyper::Client;
use self::hyper::header::Connection;
use self::hyper::header::ConnectionOption;

use Provider;
use Track;

pub fn extract_tracks(url: &str) -> Vec<Track> {
    let client = Client::new();
    let mut res = client.get(url)
        .header(Connection(vec![ConnectionOption::Close]))
        .send()
        .unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let input: Tendril<_> = FromStr::from_str(&body).unwrap();
    let dom: RcDom = parse(one_input(input), Default::default());
    let mut tracks  = Vec::new();
    walk(0, dom.document, &mut tracks);
    return tracks
}

// This is not proper HTML serialization, of course.
fn walk(indent: usize, handle: Handle, tracks: &mut Vec<Track>) {
    let node = handle.borrow();
    match node.node {
        Document         => (),
        Doctype(_, _, _) => (),
        Text(_)          => (),
        Comment(_)       => (),
        Element(ref name, ref attrs) => {
            let tag_name = name.local.as_slice();
            match extract_track(tag_name, attrs) {
                Some(track) => {
                    if !(*tracks).contains(&track) {
                        (*tracks).push(track)
                    }
                },
                None => {}
            }
        }
    }
    for child in node.children.iter() {
        walk(indent+4, child.clone(), tracks);
    }
}

fn attr(attr_name: &str, attrs: &Vec<Attribute>) -> Option<String> {
    for attr in attrs.iter() {
        if attr.name.local.as_slice() == attr_name {
            return Some(attr.value.to_string())
        }
    }
    None
}

fn extract_identifier(value: &str, regex_str: &str) -> Option<String> {
    match Regex::new(regex_str) {
        Ok(re) => match re.captures(value) {
            Some(cap) => match cap.at(1) {
                Some(str) => {
                    let strs: Vec<&str> = str.split('?').collect();
                    return Some(strs[0].to_string())
                },
                None => None
            },
            None => None
        },
        Err(_) => None
    }
}

pub fn extract_track(tag_name: &str, attrs: &Vec<Attribute>) -> Option<Track> {
    if tag_name == "iframe" {
        match attr("src", attrs) {
            Some(ref src) => {
                match extract_identifier(&src, r"www.youtube.com/embed/([^\?&].+)") {
                    Some(identifier) => {
                        return Some(Track {
                                    id: 0,
                              provider: Provider::YouTube,
                                 title: "".to_string(),
                                   url: src.to_string(),
                            identifier: identifier
                        });
                    },
                    None => ()
                }
                match extract_identifier(&src, r"api.soundcloud.com/tracks/([^\?&]+)") {
                    Some(identifier) => {
                        return Some(Track {
                                    id: 0,
                              provider: Provider::SoundCloud,
                                 title: "".to_string(),
                                   url: src.to_string(),
                            identifier: identifier
                        });
                    },
                    None => ()
                }
            },
            None => ()
        }
    } else if tag_name == "a" || tag_name == "link" {
        match attr("href", attrs) {
            Some(ref href) => {
                match extract_identifier(&href, r"www.youtube.com/watch\?v=([^\?&]+)") {
                    Some(identifier) => {
                        return Some(Track {
                                    id: 0,
                              provider: Provider::YouTube,
                                 title: "".to_string(),
                                   url: href.to_string(),
                            identifier: identifier
                        });
                    },
                    None => ()
                }
            },
            None => ()
        }
    }
    return None
}

#[cfg(test)]
mod test {
    use super::extract_identifier;

    #[test]
    fn test_extract_identifier() {
        let soundcloud_src = "https://w.soundcloud.com/player/?url=https%3A//api.soundcloud.com/tracks/195425494&auto_play=false&hide_related=false&show_comments=true&show_user=true&show_reposts=false&visual=true";
        match extract_identifier(soundcloud_src,
                                 r"api.soundcloud.com/tracks/([^\?&]+)") {
            Some(identifier) => assert_eq!(identifier,
                                           "195425494".to_string()),
            None             => assert!(false)
        }

        let youtube_src = "https://www.youtube.com/embed/X8tOngmlES0?rel=0";
        match extract_identifier(youtube_src,
                                 r"www.youtube.com/embed/([^\?&].+)") {
            Some(identifier) => assert_eq!(identifier,
                                           "X8tOngmlES0".to_string()),
            None             => assert!(false)
        }

        let youtube_href_src = "https://www.youtube.com/watch?v=oDuif301F-8";
        match extract_identifier(youtube_href_src,
                                 r"www.youtube.com/watch\?v=([^\?&]+)") {
            Some(identifier) => assert_eq!(identifier,
                                           "oDuif301F-8".to_string()),
            None             => assert!(false)
        }
    }
}
