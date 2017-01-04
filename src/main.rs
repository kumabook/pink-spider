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
extern crate chrono;

use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use std::path::Path;
use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use staticfile::Static;
use mount::Mount;
use router::{Router};
use urlencoded::UrlEncodedQuery;
use urlencoded::UrlEncodedBody;
use std::str::FromStr;
use uuid::Uuid;

#[macro_use]
extern crate string_cache;
extern crate pink_spider;

use pink_spider::error::Error;
use pink_spider::scraper::extract;
use pink_spider::model::{Track, Entry, Provider, PaginatedCollection};
use rustc_serialize::json::{ToJson, Json};
use pink_spider::youtube;
use pink_spider::soundcloud;

pub fn index_entries(req: &mut Request) -> IronResult<Response> {
    let (page, per_page) = pagination_params(req);
    let entries          = Entry::find(page, per_page);
    let json_obj: Json   = entries.to_json();
    let json_str: String = json_obj.to_string();
    Ok(Response::with((status::Ok, application_json(), json_str)))
}

pub fn index_tracks(req: &mut Request) -> IronResult<Response> {
    let (page, per_page) = pagination_params(req);
    let tracks  = Track::find(page, per_page);
    let json_obj: Json   = tracks.to_json();
    let json_str: String = json_obj.to_string();
    Ok(Response::with((status::Ok, application_json(), json_str)))
}

pub fn index_tracks_by_entry(req: &mut Request) -> IronResult<Response> {
    let ref entry_id = req.extensions.get::<Router>().unwrap().find("entry_id").unwrap();
    let uuid = try!(Uuid::parse_str(entry_id).map_err(|_| Error::Unprocessable));
    let items = Track::find_by_entry_id(uuid);
    let col = PaginatedCollection {
        page:     0,
        per_page: items.len() as i64,
        total:    items.len() as i64,
        items:    items,
    };
    let json_obj: Json   = col.to_json();
    let json_str: String = json_obj.to_string();
    Ok(Response::with((status::Ok, application_json(), json_str)))
}

pub fn playlistify(req: &mut Request) -> IronResult<Response> {
    pub fn playlistify2(req: &mut Request) -> Result<Response, Error> {
        let ref params  = try!(req.get_ref::<UrlEncodedQuery>());
        let url         = try!(params.get("url").ok_or(Error::BadRequest));
        let defaults    = &vec!("false".to_string());
        let force_param = params.get("force").unwrap_or(defaults);
        let force       = force_param.len() > 0 && &force_param[0] == "true";
        let entry       = try!(find_or_playlistify_entry(&url[0], force));
        let json_obj    = entry.to_json() as Json;
        let json_str    = json_obj.to_string() as String;
        Ok(Response::with((status::Ok, application_json(), json_str)))
    }
    playlistify2(req).map_err(|err| IronError::from(err))
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
        let mut track = try!(Track::find_or_create(t.provider, t.identifier.to_string()));
        track.title = t.title;
        let track = match t.provider {
            Provider::YouTube => match youtube::fetch_video(&t.identifier) {
                Ok(video) => track.update_with_yt_video(&video),
                Err(_)    => track.disable(),
            },
            Provider::SoundCloud => match soundcloud::fetch_track(&t.identifier) {
                Ok(sc_track) => track.update_with_sc_track(&sc_track),
                Err(_)       => track.disable(),
            },
            _ => &mut track,
        };
        try!(track.save());
        match entry.tracks.iter().find(|&t| t.id == track.id) {
            Some(_) => (),
            None    => e.add_track(track.clone())
        }
    }
    try!(e.save());
    Ok(e)
}

pub fn show_track_by_id(req: &mut Request) -> IronResult<Response> {
    let ref track_id = req.extensions.get::<Router>().unwrap()
                          .find("track_id").unwrap();
    match Track::find_by_id(track_id) {
        Ok(track) => {
            Ok(Response::with((status::Ok,
                               application_json(),
                               track.to_json().to_string())))
        },
        Err(e) => Ok(e.as_response())
    }
}

pub fn show_track(req: &mut Request) -> IronResult<Response> {
    let provider   = req.extensions.get::<Router>().unwrap().find("provider").unwrap();
    let identifier = req.extensions.get::<Router>().unwrap().find("id").unwrap();
    let p = &Provider::new(provider.to_string());
    match Track::find_by(p, identifier) {
        Ok(track) => {
            Ok(Response::with((status::Ok,
                               application_json(),
                               track.to_json().to_string())))
        },
        Err(e) => Ok(e.as_response())
    }
}


pub fn update_track(req: &mut Request) -> IronResult<Response> {
    let mut track = try!(Track::find_by_id(&query_as_string(req, "track_id")));
    match param_as_string(req, "title") {
        Some(title) => track.title = title,
        None        => println!("no title")
    }
    match param_as_string(req, "url") {
        Some(url) => track.url = url,
        None      => println!("no url")
    }
    try!(track.save());
    let res = track.to_json().to_string();
    Ok(Response::with((status::Ok, application_json(), res)))
}

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

fn pagination_params(req: &mut Request) -> (i64, i64) {
    match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref params) =>
            (params.get("page").unwrap()[0].to_string().parse::<i64>().unwrap(),
             params.get("per_page").unwrap()[0].to_string().parse::<i64>().unwrap()),
        Err(_) =>
            (0, 10)
    }
}

fn application_json() -> Mime {
    Mime::from_str("application/json").ok().unwrap()
}

pub fn main() {
    let path = Path::new("public");
    let mut mount = Mount::new();
    mount.mount("/web/", Static::new(Path::new(path)));
    let router = router!(
        playlistify:           get  "/playlistify"              => playlistify,
        show_track_by_id:      get  "/tracks/:track_id"         => show_track_by_id,
        update_track:          post "/tracks/:track_id"         => update_track,
        show_track:            get  "/tracks/:provider/:id"     => show_track,
        index_entries:         get  "/entries"                  => index_entries,
        index_tracks:          get  "/tracks"                   => index_tracks,
        index_tracks_by_entry: get  "/entries/:entry_id/tracks" => index_tracks_by_entry,
        web:                   get  "/*"                        => mount,
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
