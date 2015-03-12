#![feature(net)]
extern crate iron;

use std::net::{IpAddr, SocketAddr};
use iron::prelude::*;
use iron::status;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
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
    let ip = IpAddr::new_v4(0, 0, 0, 0);
    Iron::new(hello_world).http(SocketAddr::new(ip, port)).unwrap();
}
