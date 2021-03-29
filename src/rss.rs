use reqwest::header:: {
    HeaderMap,
    CONNECTION,
  //  CONNECTION_OPTION,
    CONTENT_TYPE,
    ACCEPT,
//    qitem,
};
use mime::Mime;
use std::io::Read;
use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_1;
use http;
use error::Error;
use error::Error::BadRequest;
use feed_rs;

fn get_charset(headers: &HeaderMap) -> Option<String> {
    headers.get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<Mime>().ok())
        .and_then(|mime| mime.get_param("charset").map(|n| n.as_str().to_string()))
}

pub fn fetch(url: &str) -> Result<feed_rs::Feed, Error> {
    let mime: Mime = "*/*".parse().unwrap();
    let client = http::client();
    let mut builder = client.get(url);
//    builder.header(Connection(vec![ConnectionOption::Close]));
//    builder.header(Accept(vec![qitem(mime)]));
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
