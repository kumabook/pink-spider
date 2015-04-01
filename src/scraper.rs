extern crate html5ever;
extern crate regex;
extern crate hyper;
extern crate string_cache;

use self::html5ever::sink::common::{Document, Doctype, Text, Comment, Element};
use self::html5ever::sink::rcdom::{RcDom, Handle};
use self::html5ever::{parse, one_input, Attribute};
use std::default::Default;
use std::io::Read;

use self::regex::Regex;
use self::hyper::Client;
use self::hyper::header::Connection;
use self::hyper::header::ConnectionOption;

use self::string_cache::namespace::{QualName, Namespace};
use self::string_cache::atom::Atom;

use Provider;
use Track;

pub fn extract_tracks(url: &str) -> Vec<Track> {
    let mut client = Client::new();
     let mut res = client.get(url)
                         .header(Connection(vec![ConnectionOption::Close]))
                         .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let dom: RcDom = parse(one_input(body), Default::default());
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
                Some(track) => (*tracks).push(track),
                None => {}
            }
        }
    }
    for child in node.children.iter() {
        walk(indent+4, child.clone(), tracks);
    }
}

pub fn extract_track(tag_name: &str, attrs: &Vec<Attribute>) -> Option<Track> {
    if tag_name == "iframe" {
        for attr in attrs.iter() {
            match Regex::new(r"www.youtube.com/embed") {
                Ok(re) =>
                    if re.is_match(&attr.value) {
                        match Regex::new(r"www.youtube.com/embed/(.+)") {
                            Ok(re) => {
                                let cap = re.captures(&attr.value).unwrap();
                                let strs: Vec<&str> = cap.at(1).unwrap().split_str('?').collect();
                                return Some(Track {
                                           id: 0,
                                     provider: Provider::YouTube,
                                        title: strs[0].to_string(),
                                          url: attr.value.to_string(),
                                   identifier: strs[0].to_string()
                                })
                            },
                            Err(_) =>
                                return None
                        }
                    },
                Err(_) =>
                    return None
            };
            match Regex::new(r"api.soundcloud.com/tracks/") {
                Ok(re) =>
                    if re.is_match(&attr.value) {
/*                        println!("SoundCloud {}=\"{}\"",
                                 attr.name.local.as_slice(),
                                 attr.value);*/
                        match Regex::new(r"api.soundcloud.com/tracks/(.+)") {
                            Ok(re) => {
                                let cap = re.captures(&attr.value).unwrap();
                                let strs: Vec<&str> = cap.at(1).unwrap().split_str('&').collect();
//                                println!("id: {} ", strs[0]);
                                return Some(Track {
                                           id: 0,
                                     provider: Provider::SoundCloud,
                                        title: strs[0].to_string(),
                                          url: attr.value.to_string(),
                                   identifier: strs[0].to_string()
                                })
                            },
                            Err(_) =>
                                return None
                        }
                    },
                Err(_) =>
                    return None
            };
        }
    } else if tag_name == "a" {
        for attr in attrs.iter() {
            let href = QualName {
                   ns: Namespace(string_cache::atom::Atom::from_slice("")),
                local: string_cache::atom::Atom::from_slice("href")
            };
            if attr.name == href {
                match Regex::new(r"www.youtube.com/watch\?v=(.+)") {
                    Ok(re) => match re.captures(&attr.value) {
                        Some(cap) => match cap.at(1) {
                            Some(str) => {
                                let strs: Vec<&str> = str.split_str('?').collect();
                                return Some(Track {
                                            id: 0,
                                      provider: Provider::YouTube,
                                         title: strs[0].to_string(),
                                           url: attr.value.to_string(),
                                    identifier: strs[0].to_string()
                                })
                            },
                            None => ()
                        },
                        None => ()
                    },
                    Err(_) => ()
                }
            }
        }
    }
    return None
}
