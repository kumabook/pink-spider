use hyper;
use hyper_native_tls;

pub fn client() -> hyper::Client {
    let tls        = hyper_native_tls::NativeTlsClient::new().unwrap();
    let connector  = hyper::net::HttpsConnector::new(tls);
    hyper::Client::with_connector(connector)
}
