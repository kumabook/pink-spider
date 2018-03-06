use reqwest;
use std::time::Duration;

pub fn client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(Duration::new(30, 0))
        .build().unwrap()
}
