#![feature(plugin, rustc_private)]
#![plugin(string_cache_plugin)]
extern crate iron;
extern crate router;
extern crate urlencoded;
extern crate serialize;
extern crate html5ever;
extern crate regex;
extern crate core;

use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use iron::prelude::*;
use iron::status;
use iron::headers::{ContentType};
use iron::modifiers::Header;
use iron::mime::Mime;
use router::{Router};
use urlencoded::UrlEncodedQuery;
use core::str::FromStr;

#[macro_use]
extern crate string_cache;

use pink_spider::scraper::extract_tracks;
use pink_spider::model::{Track, Entry, create_tables, drop_tables};
use serialize::json::{ToJson, Json};


extern crate pink_spider;

fn playlistify(req: &mut Request) -> IronResult<Response> {
    match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref params) => {
            match params.get("url") {
                Some(url) => {
                    let entry = find_or_create_entry(&url[0]);
                    let json_obj: Json = entry.to_json();
                    let json_str: String = json_obj.to_string();
                    return Ok(Response::with((status::Ok, json_str)))
                },
                None => println!("no url")
            }
        }
        Err(ref e) =>
            println!("{:?}", e)
    };

    Ok(Response::with((status::Ok, "{}")))
}

fn find_or_create_entry(url: &str) -> Entry {
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

fn show_track(req: &mut Request) -> IronResult<Response> {
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

fn main() {
    drop_tables();
    create_tables();

    let mut router = Router::new();
//    router.post(  "auth",                    signin);
    router.get(   "/playlistify",            playlistify);
//    router.get(   "/entry/:entry_id",        show_entry);
//    router.post(  "/entry/:entry_id",        update_entry);
//    router.delete("/entry/:entry_id",        destroy_entry);
//    router.put(   "/tracks/:track_id",       create_track);
    router.get(   "/tracks/:track_id",       show_track);
//    router.post(  "/tracks/:track_id",       update_track);
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
