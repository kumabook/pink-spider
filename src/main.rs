#![feature(plugin, rustc_private)]
#![plugin(string_cache_plugin)]
extern crate iron;
extern crate router;
extern crate urlencoded;
extern crate serialize;

use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use iron::prelude::*;
use iron::status;
use router::{Router};
use urlencoded::UrlEncodedQuery;

extern crate html5ever;
extern crate regex;

#[macro_use]
extern crate string_cache;

use pink_spider::scraper::extract_tracks;
use pink_spider::model::{Entry, create_tables, drop_tables};
use serialize::json::{ToJson, Json};


extern crate pink_spider;

fn log_params(req: &mut Request) -> IronResult<Response> {
    // Extract the decoded data as hashmap, using the UrlEncodedQuery plugin.
    match req.get_ref::<UrlEncodedQuery>() {
        Ok(ref params) => {
            match params.get("url") {
                Some(url) => {
                    let tracks = extract_tracks(&url[0]);
                    let entry = Entry {
                            id: 0,
                           url: url[0].to_string(),
                        tracks: tracks
                    };
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

fn main() {
    drop_tables();
    create_tables();

    let mut router = Router::new();
    router.get("/playlistify", log_params);
    router.get("/:query", handler);

    fn handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }

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
