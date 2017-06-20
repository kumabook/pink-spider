use hyper::header::Headers;
use hyper::header::Connection;
use hyper::header::ConnectionOption;
use hyper::header::Accept;
use hyper::header::{qitem};
use hyper::mime::*;
use std::io::Read;
use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_1;
use hyper::header::ContentType;
use http;
use error::Error;
use error::Error::BadRequest;
use feed_rs;

fn get_charset(headers: &Headers) -> Option<&str> {
    match headers.get::<ContentType>().map(|c| c.get_param("charset")) {
        Some(Some(&Value::Ext(ref charset))) => Some(charset),
        _                                    => None,
    }
}

pub fn fetch(url: &str) -> Result<feed_rs::Feed, Error> {
    let mime: Mime = "*/*".parse().unwrap();
    let client = http::client();
    let builder = client.get(url)
        .header(Connection(vec![ConnectionOption::Close]))
        .header(Accept(vec![qitem(mime)]));
    let mut res = try!(builder.send());
    let charset = get_charset(&res.headers).map(|v| v.to_lowercase());
    match charset.as_ref().map(String::as_ref) {
        Some("iso-8859-1") => {
            let mut body = vec![];
            try!(res.read_to_end(&mut body).map_err(|_| BadRequest));
            let decode_result = ISO_8859_1.decode(&body, DecoderTrap::Strict);
            let cell = try!(decode_result.map_err(|_| BadRequest));
            let mut s = cell.as_bytes();
            feed_rs::parser::parse(&mut s).ok_or(BadRequest)
        },
        _ => feed_rs::parser::parse(&mut res).ok_or(BadRequest),
    }
}
