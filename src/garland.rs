#![deny(warnings)]
extern crate hyper;
extern crate hyper_tls;
extern crate pretty_env_logger;

use http;
use hyper::rt::{self, Future, Stream};
use hyper::Client;
use hyper_tls::HttpsConnector;
use url::form_urlencoded;

fn main() {
    pretty_env_logger::init();

    let garland_url = String::from("https://www.garlandtools.org/api/search.php?");
    let encoded_url: String = form_urlencoded::Serializer::new(garland_url)
        .append_pair("craftable", "1")
        .append_pair("text", "rakshasa dogi of casting")
        .append_pair("lang", "en")
        .finish();
    let uri: http::Uri = encoded_url.parse().unwrap();
    rt::run(fetch_url(uri));
}

fn fetch_url(url: hyper::Uri) -> impl Future<Item = (), Error = ()> {
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder().build::<_, hyper::Body>(https);
    let mut buffer: Vec<u8> = Vec::new();

    client
        // Fetch the url...
        .get(url)
        // And then, if we get a response back...
  
        // If all good, just tell the user...
        .map(|_| {
            println!("\n\nDone.");
        })
        // If there was an error, let the user know...
        .map_err(|err| {
            eprintln!("Error {}", err);
        })
}
