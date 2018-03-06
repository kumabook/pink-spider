use reqwest::header:: {
    Headers,
    Connection,
    ConnectionOption,
    ContentType,
    Accept,
    qitem,
};
use reqwest::mime::*;
use std::io::Read;
use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_1;
use http;
use error::Error;
use error::Error::BadRequest;
use feed_rs;

fn get_charset(headers: &Headers) -> Option<&str> {
    headers.get::<ContentType>()
        .and_then(|c| c.get_param(CHARSET))
        .map(|n| n.as_str())
}

pub fn fetch(url: &str) -> Result<feed_rs::Feed, Error> {
    let mime: Mime = "*/*".parse().unwrap();
    let client = http::client();
    let mut builder = client.get(url);
    builder.header(Connection(vec![ConnectionOption::Close]));
    builder.header(Accept(vec![qitem(mime)]));
    let mut res = builder.send()?;
    let charset = get_charset(&res.headers()).map(|v| v.to_lowercase());
    match charset.as_ref().map(String::as_ref) {
        Some("iso-8859-1") => {
            let mut body = vec![];
            res.read_to_end(&mut body).map_err(|_| BadRequest)?;
            let decode_result = ISO_8859_1.decode(&body, DecoderTrap::Strict);
            let cell = decode_result.map_err(|_| BadRequest)?;
            let mut s = cell.as_bytes();
            feed_rs::parser::parse(&mut s).ok_or(BadRequest)
        },
        _ => feed_rs::parser::parse(&mut res).ok_or(BadRequest),
    }
}
