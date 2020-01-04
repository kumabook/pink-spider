extern crate iron;
#[macro_use]
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate html5ever;
extern crate regex;
extern crate uuid;
extern crate chrono;
extern crate bodyparser;
extern crate params;
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
use std::str::FromStr;
use uuid::Uuid;
use chrono::NaiveDateTime;

extern crate pink_spider;

use pink_spider::error::Error;
use pink_spider::model::{Model, Feed, Entry, Track, Playlist, PlaylistTrack, Album, Artist, Enclosure, Provider, PaginatedCollection, Filter, FilterType};
use pink_spider::get_env;
use pink_spider::rss;

const DEFAULT_PER_PAGE: i64 = 25;

fn to_err<E>(e: E) -> Error
    where pink_spider::error::Error: std::convert::From<E> {
    Error::from(e)
}

pub fn index<'a, T: Model<'a>>(req: &mut Request) -> IronResult<Response> {
    let (page, per_page) = pagination_params(req);
    let items = if let Ok(q) = param_as_string(req, "query") {
        let q = format!("%{}%", q);
        let filter = Filter {
            filter_type: FilterType::Contains,
            field: T::search_prop(),
            value: &q,
        };
        T::find(page, per_page, Some(filter))
    } else if let Ok(q) = param_as_string(req, "type") {
        if q == "active" {
            T::find(page, per_page, Some(Filter {
                filter_type: FilterType::GreaterThan,
                field: "velocity",
                value: &0.0,
            }))
        } else {
            T::find(page, per_page, None)
        }
    } else {
        T::find(page, per_page, None)
    };
    let body  = serde_json::to_string(&items).map_err(to_err)?;
    Ok(Response::with((status::Ok, application_json(), body)))
}

pub fn index_entries(req: &mut Request) -> IronResult<Response> {
    pub fn index_entries2(req: &mut Request) -> Result<Response, Error> {
        let (page, per_page) = pagination_params(req);
        let url              = param_as_string(req, "feed_url");
        let newer_than       = param_as_string(req, "newer_than");
        let entries = if let (Ok(url), Ok(newer_than)) = (url, newer_than) {
            let feed = Feed::find_by_url(&url)?;
            let newer_than = newer_than.parse::<i64>()
                .map(|t| NaiveDateTime::from_timestamp(t, 0)).ok();
            Entry::find_by_feed_id(feed.id, newer_than, page, per_page)
        } else {
            Entry::find(page, per_page, None)
        };
        let body = serde_json::to_string(&entries).map_err(to_err)?;
        Ok(Response::with((status::Ok, application_json(), body)))
    }
    index_entries2(req).map_err(|err| IronError::from(err))
}

pub fn index_entries_by_feed(req: &mut Request) -> IronResult<Response> {
    let (page, per_page) = pagination_params(req);
    let ref id           = req.extensions.get::<Router>().unwrap().find("id").unwrap();
    let feed             = Feed::find_by_id(id)?;
    let entries          = Entry::find_by_feed_id(feed.id, None, page, per_page);
    let body             = serde_json::to_string(&entries).map_err(to_err)?;
    Ok(Response::with((status::Ok, application_json(), body)))
}

pub fn mget<'a, T: Model<'a>>(req: &mut Request) -> IronResult<Response> {
    let ids              = params_as_uuid_array(req)?;
    let items            = T::mget(ids)?;
    let body             = serde_json::to_string(&items).map_err(to_err)?;
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
    let uuid = Uuid::parse_str(entry_id).map_err(|_| Error::Unprocessable)?;
    let items = T::find_by_entry_id(uuid);
    let col = PaginatedCollection {
        page:     0,
        per_page: items.len() as i64,
        total:    items.len() as i64,
        items:    items,
    };
    let body = serde_json::to_string(&col).map_err(to_err)?;
    Ok(Response::with((status::Ok, application_json(), body)))
}

pub fn index_tracks_by_playlist(req: &mut Request) -> IronResult<Response> {
    let ref id = req.extensions.get::<Router>().unwrap().find("playlist_id").unwrap();
    let uuid   = Uuid::parse_str(id).map_err(|_| Error::Unprocessable)?;
    let map  = PlaylistTrack::find_by_playlist_ids(vec![uuid])?;
    let items = map.get(&uuid).unwrap().clone();
    let col = PaginatedCollection {
        page:     0,
        per_page: items.len() as i64,
        total:    items.len() as i64,
        items:    items,
    };
    let body = serde_json::to_string(&col).map_err(to_err)?;
    Ok(Response::with((status::Ok, application_json(), body)))
}

pub fn legacy_playlistify(req: &mut Request) -> IronResult<Response> {
    pub fn playlistify2(req: &mut Request) -> Result<Response, Error> {
        let url         = param_as_string(req, "url")?;
        let force       = param_as_string(req, "force").unwrap_or("false".to_string());
        let mut entry   = find_or_playlistify_entry(&url, &force == "true")?;
        entry.tracks    = entry.tracks
            .iter()
            .filter(|t| t.provider == Provider::YouTube || t.provider == Provider::SoundCloud)
            .map(|t| t.clone())
            .collect();
        let body = serde_json::to_string(&entry).map_err(to_err)?;
        Ok(Response::with((status::Ok, application_json(), body)))
    }
    playlistify2(req).map_err(|err| IronError::from(err))
}

pub fn playlistify(req: &mut Request) -> IronResult<Response> {
    pub fn playlistify2(req: &mut Request) -> Result<Response, Error> {
        let url         = param_as_string(req, "url")?;
        let force       = param_as_string(req, "force").unwrap_or("false".to_string());
        let entry       = find_or_playlistify_entry(&url, &force == "true")?;
        let body        = serde_json::to_string(&entry).map_err(to_err)?;
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
                entry.playlistify()?;
                Ok(entry)
            } else {
                Ok(entry)
            }
        },
        Err(_) => {
            let mut entry = Entry::create_by_url(url.to_string())?;
            let _ = entry.playlistify();
            println!("Create new entry to database cache: {}", url);
            Ok(entry)
        },
    }
}

pub fn update_entry(req: &mut Request) -> IronResult<Response> {
    let mut entry = Entry::find_by_id(&query_as_string(req, "id"))?;
    entry.playlistify()?;
    entry.save()?;
    let body = serde_json::to_string(&entry).map_err(to_err)?;
    Ok(Response::with((status::Ok, application_json(), body)))
}

pub fn update<'a, T: Enclosure<'a>>(req: &mut Request) -> IronResult<Response> {
    let mut enclosure = T::find_by_id(&query_as_string(req, "id"))?;
    let map = req.get_ref::<params::Params>().map_err(to_err)?;
    let _ = enclosure.fetch_props();
    enclosure.update_attributes(map);
    enclosure.save()?;
    let body = serde_json::to_string(&enclosure).map_err(to_err)?;
    Ok(Response::with((status::Ok, application_json(), body)))
}

pub fn show<'a, T: Enclosure<'a>>(req: &mut Request) -> IronResult<Response> {
    let provider   = req.extensions.get::<Router>().unwrap().find("provider").unwrap();
    let identifier = req.extensions.get::<Router>().unwrap().find("id").unwrap();
    let p = &Provider::new(provider.to_string());
    let enclosure = T::find_by(p, identifier)?;
    let body      = serde_json::to_string(&enclosure).map_err(to_err)?;
    Ok(Response::with((status::Ok, application_json(), body)))
}

pub fn show_by_id<'a, T: Model<'a>>(req: &mut Request) -> IronResult<Response> {
    let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap();
    let mut enclosures = vec![T::find_by_id(id)?];
    T::set_relations(&mut enclosures)?;
    let body      = serde_json::to_string(&enclosures[0]).map_err(to_err)?;
    Ok(Response::with((status::Ok, application_json(), body)))
}

pub fn create<'a, T: Enclosure<'a>>(req: &mut Request) -> IronResult<Response> {
    let identifier     = param_as_string(req, "identifier")?;
    let provider       = param_as_string(req, "provider")?;
    let owner_id       = param_as_string(req, "owner_id").ok();
    let url            = param_as_string(req, "url").ok();
    let p              = Provider::new(provider.to_string());
    let mut enclosure  = T::find_or_create(p, identifier.to_string())?;
    enclosure.set_owner_id(owner_id);
    if let Some(url) = url {
        enclosure.set_url(url);
    }
    enclosure.fetch_props()?;
    enclosure.save()?;
    let body = serde_json::to_string(&enclosure).map_err(to_err)?;
    Ok(Response::with((status::Ok, application_json(), body)))
}

pub fn create_feed_by_url(req: &mut Request) -> IronResult<Response> {
    let json = req.get::<bodyparser::Json>()
        .map_err(|_| IronError::from(Error::Unprocessable))
        .and_then(|v| v.ok_or(IronError::from(Error::Unprocessable)))?;
    let url = json.get("url")
        .and_then(|v| v.as_str())
        .ok_or(IronError::from(Error::Unprocessable))?;
    println!("--------------- Creating {:?} ---------------", url);
    let rss_feed = rss::fetch(url)?;
    let mut item = Feed::find_or_create_by_url(url.to_string())?;
    item.update_props(rss_feed);
    println!("Result {:?}  {:?}", url, item);
    item.save()?;
    println!("--------------- Created {:?} ---------------", url);
    let body = serde_json::to_string(&item).map_err(to_err)?;
    Ok(Response::with((status::Ok, application_json(), body)))
}

fn param_as_string(req: &mut Request, key: &str) -> Result<String, Error> {
    let map = req.get_ref::<params::Params>().map_err(to_err)?;
    match map.find(&[key]) {
        Some(&params::Value::String(ref value)) => Ok(value.to_string()),
        _                                       => Err(Error::BadRequest),
    }
}

fn query_as_string(req: &mut Request, key: &str) -> String {
    return req.extensions.get::<Router>().unwrap()
        .find(key).unwrap().to_string();
}

fn pagination_params(req: &mut Request) -> (i64, i64) {
    let page = param_as_string(req, "page")
        .and_then(|v| v.to_string().parse::<i64>().map_err(to_err))
        .unwrap_or(0);
    let per_page = param_as_string(req, "per_page")
        .and_then(|v| v.to_string().parse::<i64>().map_err(to_err))
        .unwrap_or(DEFAULT_PER_PAGE);
    (page, per_page)
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

        index_feeds:              get  "/v1/feeds"                       => index::<Feed>,
        show_feed:                get  "/v1/feeds/:id"                   => show_by_id::<Feed>,
        mget_feeds:               get  "/v1/feeds/.mget"                 => mget::<Feed>,
        create_feed:              post "/v1/feeds"                       => create_feed_by_url,

        index_entries:            get  "/v1/entries"                     => index_entries,
        index_entries_by_feed:    get  "/v1/feeds/:id/entries"           => index_entries_by_feed,
        show_entry:               get  "/v1/entries/:id"                 => show_by_id::<Entry>,
        update_entry:             post "/v1/entries/:id"                 => update_entry,

        show_artist_by_id:        get  "/v1/artists/:id"                 => show_by_id::<Artist>,
        show_artist:              get  "/v1/artists/:provider/:id"       => show::<Artist>,
        mget_artists:             post "/v1/artists/.mget"               => mget::<Artist>,
        create_artist:            post "/v1/artists"                     => create::<Artist>,
        update_artist:            post "/v1/artists/:id"                 => update::<Artist>,
        index_artists:            get  "/v1/artists"                     => index::<Artist>,

        show_track_by_id:         get  "/v1/tracks/:id"                  => show_by_id::<Track>,
        show_track:               get  "/v1/tracks/:provider/:id"        => show::<Track>,
        mget_tracks:              post "/v1/tracks/.mget"                => mget::<Track>,
        create_track:             post "/v1/tracks"                      => create::<Track>,
        update_track:             post "/v1/tracks/:id"                  => update::<Track>,
        index_tracks:             get  "/v1/tracks"                      => index::<Track>,
        index_tracks_by_entry:    get  "/v1/entries/:entry_id/tracks"    => index_by_entry::<Track>,
        index_tracks_by_playlist: get  "/v1/playlists/:playlist_id/tracks"  => index_tracks_by_playlist,

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
