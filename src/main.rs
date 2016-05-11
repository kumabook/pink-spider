extern crate iron;
extern crate router;
extern crate urlencoded;
extern crate rustc_serialize;
extern crate html5ever;
extern crate regex;
extern crate uuid;

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

#[macro_use]
extern crate string_cache;

extern crate pink_spider;

use pink_spider::error::Error;
use pink_spider::scraper::extract_tracks;
use pink_spider::model::{Track, Entry, Provider};
use rustc_serialize::json::{ToJson, Json};


pub fn playlistify(req: &mut Request) -> IronResult<Response> {
    let json_type = Header(ContentType(Mime::from_str("application/json").ok().unwrap()));
    match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref params) => {
            match params.get("url") {
                Some(url) => {
                    match find_or_create_entry(&url[0]) {
                        Ok(entry) => {
                            let json_obj: Json   = entry.to_json();
                            let json_str: String = json_obj.to_string();
                            Ok(Response::with((status::Ok, json_type, json_str)))
                        },
                        Err(e) => Ok(e.as_response())
                    }
                },
                None => Ok(Error::BadRequest.as_response())
            }
        }
        Err(_) => Ok(Error::BadRequest.as_response())
    }
}

pub fn find_or_create_entry(url: &str) -> Result<Entry, Error> {
    match Entry::find_by_url(url) {
        Ok(entry) => {
            println!("Get entry from database cache");
            Ok(entry)
        },
        Err(_) => {
            let     tracks = try!(extract_tracks(url));
            let mut entry  = try!(Entry::create_by_url(url.to_string()));
            println!("Create new entry to database cache");
            for t in tracks {
                let track = try!(Track::find_or_create(t.provider, t.title, t.url, t.identifier));
                entry.add_track(track)
            }
            Ok(entry)
        },
    }
}

pub fn show_track(req: &mut Request) -> IronResult<Response> {
    let ref track_id = req.extensions.get::<Router>().unwrap()
                          .find("track_id").unwrap();
    let json_type = Header(ContentType(Mime::from_str("application/json").ok().unwrap()));
    match Track::find_by_id(track_id) {
        Ok(track) => {
            Ok(Response::with((status::Ok,
                               json_type,
                               track.to_json().to_string())))
        },
        Err(e) => Ok(e.as_response())
    }
}

pub fn show_track_by_provider_id(req: &mut Request) -> IronResult<Response> {
    let provider   = req.extensions.get::<Router>().unwrap().find("provider").unwrap();
    let identifier = req.extensions.get::<Router>().unwrap().find("id").unwrap();
    let json_type  = Header(ContentType(Mime::from_str("application/json").ok().unwrap()));
    let p = &Provider::new(provider.to_string());
    match Track::find_by(p, identifier) {
        Ok(track) => {
            Ok(Response::with((status::Ok,
                               json_type,
                               track.to_json().to_string())))
        },
        Err(e) => Ok(e.as_response())
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

    match Track::find_by_id(&query_as_string(req, "track_id")) {
        Ok(mut track) => {
            match param_as_string(req, "title") {
                Some(title) => track.title = title,
                None        => println!("no title")
            }
            match param_as_string(req, "url") {
                Some(url) => track.url = url,
                None      => println!("no url")
            }
            match track.save() {
                Ok(_) => Ok(Response::with((status::Ok,
                                         json_type,
                                         track.to_json().to_string()))),
                Err(e) =>  Ok(e.as_response())
            }
        },
        Err(e) =>  {
            Ok(e.as_response())
        }
    }
}

pub fn main() {
    let mut router = Router::new();
    router.get( "/playlistify"         , playlistify);
    router.get( "/tracks/:track_id"    , show_track);
    router.post("/tracks/:track_id"    , update_track);
    router.get( "/tracks/:provider/:id", show_track_by_provider_id);

    let port_str = match std::env::var("PORT") {
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
