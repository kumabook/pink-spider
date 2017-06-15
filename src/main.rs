extern crate iron;
#[macro_use]
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate urlencoded;
extern crate html5ever;
extern crate regex;
extern crate uuid;
extern crate chrono;
extern crate bodyparser;
extern crate serde;
extern crate serde_json;

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

extern crate pink_spider;

use pink_spider::error::Error;
use pink_spider::scraper::extract;
use pink_spider::model::{Model, Track, Playlist, Album, Artist, Entry, Enclosure, Provider, PaginatedCollection};
use pink_spider::get_env;

fn to_err(e: serde_json::Error) -> Error { Error::from(e) }

pub fn index_entries(req: &mut Request) -> IronResult<Response> {
    let (page, per_page) = pagination_params(req);
    let entries          = Entry::find(page, per_page);
    let body             = try!(serde_json::to_string(&entries).map_err(to_err));
    Ok(Response::with((status::Ok, application_json(), body)))
}

pub fn index<'a, T: Model<'a>>(req: &mut Request) -> IronResult<Response> {
    let (page, per_page) = pagination_params(req);
    let enclosures       = T::find(page, per_page);
    let body             = try!(serde_json::to_string(&enclosures).map_err(to_err));
    Ok(Response::with((status::Ok, application_json(), body)))
}

pub fn mget<'a, T: Model<'a>>(req: &mut Request) -> IronResult<Response> {
    let ids              = try!(params_as_uuid_array(req));
    let items            = try!(T::mget(ids));
    let body             = try!(serde_json::to_string(&items).map_err(to_err));
    Ok(Response::with((status::Ok, application_json(), body)))
}

fn params_as_uuid_array(req: &mut Request) -> IronResult<Vec<Uuid>> {
    match req.get::<bodyparser::Json>() {
        Ok(Some(serde_json::Value::Array(ids))) =>
            Ok(ids
               .iter()
               .map(|v| Uuid::parse_str(v.as_str().unwrap_or("")))
               .filter(|opt| opt.is_ok())
               .map(|opt| opt.unwrap())
               .collect()),
        _ => Err(IronError::from(Error::Unprocessable)),
    }
}

pub fn index_by_entry<'a, T: Enclosure<'a>>(req: &mut Request) -> IronResult<Response> {
    let ref entry_id = req.extensions.get::<Router>().unwrap().find("entry_id").unwrap();
    let uuid = try!(Uuid::parse_str(entry_id).map_err(|_| Error::Unprocessable));
    let items = T::find_by_entry_id(uuid);
    let col = PaginatedCollection {
        page:     0,
        per_page: items.len() as i64,
        total:    items.len() as i64,
        items:    items,
    };
    let body = try!(serde_json::to_string(&col).map_err(to_err));
    Ok(Response::with((status::Ok, application_json(), body)))
}

pub fn legacy_playlistify(req: &mut Request) -> IronResult<Response> {
    pub fn playlistify2(req: &mut Request) -> Result<Response, Error> {
        let ref params  = try!(req.get_ref::<UrlEncodedQuery>());
        let url         = try!(params.get("url").ok_or(Error::BadRequest));
        let defaults    = &vec!("false".to_string());
        let force_param = params.get("force").unwrap_or(defaults);
        let force       = force_param.len() > 0 && &force_param[0] == "true";
        let mut entry   = try!(find_or_playlistify_entry(&url[0], force));
        entry.tracks    = entry.tracks
            .iter()
            .filter(|t| t.provider == Provider::YouTube || t.provider == Provider::SoundCloud)
            .map(|t| t.clone())
            .collect();
        let body = try!(serde_json::to_string(&entry).map_err(to_err));
        Ok(Response::with((status::Ok, application_json(), body)))
    }
    playlistify2(req).map_err(|err| IronError::from(err))
}

pub fn playlistify(req: &mut Request) -> IronResult<Response> {
    pub fn playlistify2(req: &mut Request) -> Result<Response, Error> {
        let ref params  = try!(req.get_ref::<UrlEncodedQuery>());
        let url         = try!(params.get("url").ok_or(Error::BadRequest));
        let defaults    = &vec!("false".to_string());
        let force_param = params.get("force").unwrap_or(defaults);
        let force       = force_param.len() > 0 && &force_param[0] == "true";
        let entry       = try!(find_or_playlistify_entry(&url[0], force));
        let body        = try!(serde_json::to_string(&entry).map_err(to_err));
        Ok(Response::with((status::Ok, application_json(), body)))
    }
    playlistify2(req).map_err(|err| IronError::from(err))
}

pub fn find_or_playlistify_entry(url: &str, force: bool) -> Result<Entry, Error> {
    match Entry::find_by_url(url) {
        Ok(mut entry) => {
            println!("Get entry from database cache: {}", url);
            if force {
                println!("Update entry: {}", url);
                try!(entry.playlistify());
                Ok(entry)
            } else {
                Ok(entry)
            }
        },
        Err(_) => {
            let mut entry = try!(Entry::create_by_url(url.to_string()));
            try!(entry.playlistify());
            println!("Create new entry to database cache: {}", url);
            Ok(entry)
        },
    }
}

pub fn update<'a, T: Enclosure<'a>>(req: &mut Request) -> IronResult<Response> {
    let mut enclosure = try!(T::find_by_id(&query_as_string(req, "id")));
    try!(enclosure.fetch_props());
    try!(enclosure.save());
    let body = try!(serde_json::to_string(&enclosure).map_err(to_err));
    Ok(Response::with((status::Ok, application_json(), body)))
}

pub fn show<'a, T: Enclosure<'a>>(req: &mut Request) -> IronResult<Response> {
    let provider   = req.extensions.get::<Router>().unwrap().find("provider").unwrap();
    let identifier = req.extensions.get::<Router>().unwrap().find("id").unwrap();
    let p = &Provider::new(provider.to_string());
    let enclosure = try!(T::find_by(p, identifier));
    let body      = try!(serde_json::to_string(&enclosure).map_err(to_err));
    Ok(Response::with((status::Ok, application_json(), body)))
}

pub fn show_by_id<'a, T: Model<'a>>(req: &mut Request) -> IronResult<Response> {
    let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap();
    let enclosure = try!(T::find_by_id(id));
    let body      = try!(serde_json::to_string(&enclosure).map_err(to_err));
    Ok(Response::with((status::Ok, application_json(), body)))
}

pub fn create<'a, T: Enclosure<'a>>(req: &mut Request) -> IronResult<Response> {
    let identifier     = try!(param_as_string(req, "identifier").ok_or(Error::BadRequest));
    let provider       = try!(param_as_string(req, "provider").ok_or(Error::BadRequest));
    let owner_id       = param_as_string(req, "owner_id");
    let url            = param_as_string(req, "url");
    let p              = Provider::new(provider.to_string());
    let mut enclosure  = try!(T::find_or_create(p, identifier.to_string()));
    enclosure.set_owner_id(owner_id);
    if let Some(url) = url {
        enclosure.set_url(url);
    }
    try!(enclosure.fetch_props());
    try!(enclosure.save());
    let body = try!(serde_json::to_string(&enclosure).map_err(to_err));
    Ok(Response::with((status::Ok, application_json(), body)))
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
        web:                      get  "/*"                        => mount,
        legacy_playlistify:       get  "/playlistify"                    => legacy_playlistify,
        playlistify:              get  "/v1/playlistify"                 => playlistify,
        index_entries:            get  "/v1/entries"                     => index_entries,
        index_artists:            get  "/v1/artists"                     => index::<Artist>,
        mget_artists:             post "/v1/artists/.mget"               => mget::<Artist>,

        show_track_by_id:         get  "/v1/tracks/:id"                  => show_by_id::<Track>,
        show_track:               get  "/v1/tracks/:provider/:id"        => show::<Track>,
        mget_tracks:              post "/v1/tracks/.mget"                => mget::<Track>,
        create_track:             post "/v1/tracks"                      => create::<Track>,
        update_track:             post "/v1/tracks/:id"                  => update::<Track>,
        index_tracks:             get  "/v1/tracks"                      => index::<Track>,
        index_tracks_by_entry:    get  "/v1/entries/:entry_id/tracks"    => index_by_entry::<Track>,

        show_playlist_by_id:      get  "/v1/playlists/:id"               => show_by_id::<Playlist>,
        show_playlist:            get  "/v1/playlists/:provider/:id"     => show::<Playlist>,
        create_playlist:          post "/v1/playlists"                   => create::<Playlist>,
        update_playlist:          post "/v1/playlists/:id"               => update::<Playlist>,
        mget_playlists:           post "/v1/playlists/.mget"             => mget::<Playlist>,
        index_playlists:          get  "/v1/playlists"                   => index::<Playlist>,
        index_playlists_by_entry: get  "/v1/entries/:entry_id/playlists" => index_by_entry::<Playlist>,

        show_album_by_id:         get  "/v1/albums/:id"                  => show_by_id::<Album>,
        show_album:               get  "/v1/albums/:provider/:id"        => show::<Album>,
        mget_albums:              post "/v1/albums/.mget"                => mget::<Album>,
        create_album:             post "/v1/albums"                      => create::<Album>,
        update_album:             post "/v1/albums/:id"                  => update::<Album>,
        index_albums:             get  "/v1/albums"                      => index::<Album>,
        index_albums_by_entry:    get  "/v1/entries/:entry_id/albums"    => index_by_entry::<Album>,
    );
    let port_str = match get_env::var("PORT") {
        Some(n) => n,
        None    => "8080".to_string()
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
