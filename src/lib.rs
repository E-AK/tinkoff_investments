pub mod schemas;
mod services;


use hyper::Client;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;


pub struct Service {
    pub token: String,
    pub broker_account_id: String,
    http_client: Client<HttpsConnector<HttpConnector>>
}
