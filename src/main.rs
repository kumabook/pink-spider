extern crate iron;
extern crate router;
extern crate urlencoded;
extern crate rustc_serialize;
extern crate html5ever;
extern crate regex;

use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use iron::prelude::*;
use iron::status;
use iron::headers::{ContentType};
use iron::modifiers::Header;
use iron::mime::Mime;
use router::{Router};
use urlencoded::UrlEncodedQuery;
use urlencoded::UrlEncodedBody;
use std::str::FromStr;
use std::collections::BTreeMap;

#[macro_use]
extern crate string_cache;

use pink_spider::scraper::extract_tracks;
use pink_spider::model::{Track, Entry, create_tables};
use rustc_serialize::json::{ToJson, Json};


extern crate pink_spider;

pub fn playlistify(req: &mut Request) -> IronResult<Response> {
    let json_type = Header(ContentType(Mime::from_str("application/json").ok().unwrap()));
    match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref params) => {
            match params.get("url") {
                Some(url) => {
                    let entry = find_or_create_entry(&url[0]);
                    let json_obj: Json = entry.to_json();
                    let json_str: String = json_obj.to_string();
                    return Ok(Response::with((status::Ok, json_type, json_str)))
                },
                None => println!("no url")
            }
        }
        Err(ref e) =>
            println!("{:?}", e)
    };

    Ok(Response::with((status::Ok, "{}")))
}

pub fn find_or_create_entry(url: &str) -> Entry {
    match Entry::find_by_url(url) {
        Some(entry) => {
            println!("Get entry from database cache");
            entry
        },
        None => {
            let tracks = extract_tracks(url);
            match Entry::create_by_url(url.to_string()) {
                Some(mut entry) => {
                    println!("Create new entry to database cache");
                    for t in tracks {
                        match Track::create(t.provider, t.title, t.url, t.identifier) {
                            Some(track) => entry.add_track(track),
                            None        => ()
                        }
                    }
                    entry
                },
                None => {
                    println!("Failed to create entry database cache");
                    Entry {
                            id: 0,
                           url: url.to_string(),
                        tracks: tracks
                    }
                }
            }
        }
    }
}

pub fn show_track(req: &mut Request) -> IronResult<Response> {
    let ref track_id = req.extensions.get::<Router>().unwrap()
                          .find("track_id").unwrap();
    let tid: Option<i32> = track_id.parse().ok();
    let json_type = Header(ContentType(Mime::from_str("application/json").ok().unwrap()));
    match tid {
        Some(n) =>
            match Track::find_by_id(n) {
                Some(track) => {
                    let res = Response::with((status::Ok,
                                              json_type,
                                              track.to_json().to_string()));
                    Ok(res)
                },
                None =>
                    Ok(Response::with((status::Ok, json_type, "{}")))
            },
        None => Ok(Response::with((status::Ok, json_type, "{}")))
    }
}


pub fn update_track(req: &mut Request) -> IronResult<Response> {
    let json_type = Header(ContentType(Mime::from_str("application/json").ok().unwrap()));
    fn param_as_string(req: &mut Request, key: &str) -> Option<String> {
        match req.get_ref::<UrlEncodedBody>() {
            Ok(ref params) => match params.get(key) {
                Some(val) => Some(val[0].clone()),
                None      => None
            },
            Err(_) => None
        }
    }

    fn query_as_string(req: &mut Request, key: &str) -> String {
        return req.extensions.get::<Router>().unwrap()
                  .find(key).unwrap().to_string();
    }

    let track_id = query_as_string(req, "track_id");
    let tid: Option<i32> = track_id.parse().ok();

    let opt_track = match tid {
        Some(id) => Track::find_by_id(id),
        None     => None
    };

    match opt_track {
        Some(mut track) => {
            match param_as_string(req, "title") {
                Some(title) => track.title = title,
                None        => println!("no title")
            }
            match param_as_string(req, "url") {
                Some(url) => track.url = url,
                None      => println!("no url")
            }
            if track.save() {
                let res = Response::with((status::Ok,
                                          json_type,
                                          track.to_json().to_string()));
                Ok(res)
            } else {
                let mut info = BTreeMap::new();
                info.insert("error".to_string(), "Failed to save".to_string());
                Ok(Response::with((status::BadRequest,
                                   json_type,
                                   info.to_json().to_string())))
            }
        },
        None =>  {
            let mut info = BTreeMap::new();
            info.insert("error".to_string(), "Track is not found".to_string());
            Ok(Response::with((status::NotFound,
                               json_type,
                               info.to_json().to_string())))
        }
    }
}

pub fn main() {
    create_tables();

    let mut router = Router::new();
//    router.post(  "auth",                    signin);
    router.get(   "/playlistify",            playlistify);
//    router.get(   "/entry/:entry_id",        show_entry);
//    router.post(  "/entry/:entry_id",        update_entry);
//    router.delete("/entry/:entry_id",        destroy_entry);
//    router.put(   "/tracks/:track_id",       create_track);
    router.get(   "/tracks/:track_id",       show_track);
    router.post(  "/tracks/:track_id",       update_track);
//    router.delete("/tracks/:track_id",       destroy_track);
//    router.post(  "/entry/:entry_id/tracks", add_track);*/

    let opt_port = std::env::var("PORT");
    let port_str = match opt_port {
        Ok(n)    => n,
        Err(_) => "8080".to_string()
    };
    let port: u16 = match port_str.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Faild to parse port");
            return;
        }
    };
    println!("PORT {}", port_str);
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    Iron::new(router).http(SocketAddrV4::new(ip, port)).unwrap();
}
