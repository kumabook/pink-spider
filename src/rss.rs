use hyper::header::Connection;
use hyper::header::ConnectionOption;
use hyper::header::Accept;
use hyper::header::{qitem};
use hyper::mime::*;


use http;
use error::Error;
use feed_rs;

pub fn fetch(url: &str) -> Result<feed_rs::Feed, Error> {
    let mime: Mime = "*/*".parse().unwrap();
    let client = http::client();
    let builder = client.get(url)
        .header(Connection(vec![ConnectionOption::Close]))
        .header(Accept(vec![qitem(mime)]));
    let mut res = try!(builder.send());
    println!("response status: {}", res.status);
    feed_rs::parser::parse(&mut res).ok_or(Error::BadRequest)
}
