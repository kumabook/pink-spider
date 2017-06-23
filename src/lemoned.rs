use std::io::Read;
use chrono::{DateTime, UTC};
use hyper::header::{
    Headers,
    Connection,
};
use serde_json;
use get_env;
use http;


lazy_static! {
    static ref BASE_URL: String = {
        get_env::var("CUSTOM_BASE_URL").unwrap_or("http://localhost:4000".to_string())
    };
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id:                i64,
    pub artist_id:         Option<i64>,
    pub url:               String,
    pub title:             Option<String>,
    pub description:       Option<String>,
    pub thumbnail_url:     Option<String>,
    pub artwork_url:       Option<String>,
    pub audio_url:         Option<String>,
    pub duration:          Option<i64>,
    pub published_at:      DateTime<UTC>,
    pub created_at:        DateTime<UTC>,
    pub updated_at:        DateTime<UTC>,
    pub artist:            Option<Artist>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id:            i64,
    pub name:          String,
    pub url:           Option<String>,
    pub thumbnail_url: Option<String>,
    pub artwork_url:   Option<String>,
}

pub fn fetch_track(id: &str) -> serde_json::Result<Track> {
    let path = format!("/tracks/{}", id);
    fetch(&path).and_then(|s| serde_json::from_str(&s))
}

fn fetch(path: &str) -> serde_json::Result<String> {
    let url    = format!("{}/v1{}", *BASE_URL, path);
    let mut headers = Headers::new();
    headers.set(Connection::close());
    let mut res = http::client().get(&url)
                                .headers(headers)
                                .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    Ok(body)
}
