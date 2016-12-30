extern crate iron;
#[macro_use]
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate urlencoded;
extern crate rustc_serialize;
extern crate html5ever;
extern crate regex;
extern crate uuid;

use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use std::path::Path;
use iron::prelude::*;
use iron::status;
use iron::headers::{ContentType};
use iron::modifiers::Header;
use iron::mime::Mime;
use staticfile::Static;
use mount::Mount;
use router::{Router};
use urlencoded::UrlEncodedQuery;
use urlencoded::UrlEncodedBody;
use std::str::FromStr;

#[macro_use]
extern crate string_cache;

extern crate pink_spider;

use pink_spider::error::Error;
use pink_spider::scraper::extract;
use pink_spider::model::{Track, Entry, Provider};
use rustc_serialize::json::{ToJson, Json};

static JSON: &'static str = "application/json";

pub fn index_entries(req: &mut Request) -> IronResult<Response> {
    let json_type = Header(ContentType(Mime::from_str(JSON).ok().unwrap()));
    let entries = Entry::find();
    let json_obj: Json   = entries.to_json();
    let json_str: String = json_obj.to_string();
    Ok(Response::with((status::Ok, json_type, json_str)))
}

pub fn index_tracks(req: &mut Request) -> IronResult<Response> {
    let json_type = Header(ContentType(Mime::from_str(JSON).ok().unwrap()));
    let tracks = Track::find();
    let json_obj: Json   = tracks.to_json();
    let json_str: String = json_obj.to_string();
    Ok(Response::with((status::Ok, json_type, json_str)))
}

pub fn playlistify(req: &mut Request) -> IronResult<Response> {
    let json_type = Header(ContentType(Mime::from_str("application/json").ok().unwrap()));
    match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref params) => {
            match params.get("url") {
                Some(url) => {
                    let defaults = &vec!("false".to_string());
                    let force    = params.get("force").unwrap_or(defaults);
                    match find_or_playlistify_entry(&url[0], force.len() > 0 && &force[0] == "true") {
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

pub fn find_or_playlistify_entry(url: &str, force: bool) -> Result<Entry, Error> {
    match Entry::find_by_url(url) {
        Ok(entry) => {
            println!("Get entry from database cache: {}", url);
            if force {
                println!("Update entry: {}", url);
                let entry = try!(playlistify_entry(entry));
                Ok(entry)
            } else {
                Ok(entry)
            }
        },
        Err(_) => {
            let entry = try!(Entry::create_by_url(url.to_string()));
            let entry = try!(playlistify_entry(entry));
            println!("Create new entry to database cache: {}", url);
            Ok(entry)
        },
    }
}

pub fn playlistify_entry(entry: Entry) -> Result<Entry, Error> {
    let mut e = entry.clone();
    let product = try!(extract(&entry.url));
    match product.og_obj {
        Some(og_obj) => {
            e.title       = Some(og_obj.title);
            e.description = og_obj.description;
            e.locale      = og_obj.locale;
            e.visual_url  = og_obj.images.first().map(|i| i.url.clone());
        },
        None => (),
    }
    for t in product.tracks {
        let track = try!(Track::find_or_create(t.provider,
                                               t.title,
                                               t.url,
                                               t.identifier));
        match entry.tracks.iter().find(|&t| t.id == track.id) {
            Some(_) => (),
            None    => e.add_track(track.clone())
        }
    }
    try!(e.save());
    Ok(e)
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
    let path = Path::new("public");
    let mut mount = Mount::new();
    mount.mount("/web/", Static::new(Path::new(path)));
    let router = router!(
                         playlistify: get  "/playlistify"          => playlistify,
                          show_track: get  "/tracks/:track_id"     => show_track,
                        update_track: post "/tracks/:track_id"     => update_track,
           show_track_by_provider_id: get  "/tracks/:provider/:id" => show_track_by_provider_id,
                       index_entries: get  "/entries"              => index_entries,
                        index_tracks: get  "/tracks"               => index_tracks,
                                 web: get  "/*"                    => mount,
    );
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
