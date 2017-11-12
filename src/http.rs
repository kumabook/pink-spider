use hyper;
use hyper_native_tls;
use std::time::Duration;

pub fn client() -> hyper::Client {
    let tls        = hyper_native_tls::NativeTlsClient::new().unwrap();
    let connector  = hyper::net::HttpsConnector::new(tls);
    let mut client = hyper::Client::with_connector(connector);
    client.set_read_timeout(Some(Duration::new(30, 0)));
    client
}
