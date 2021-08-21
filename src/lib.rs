pub mod schemas;
mod services;


use hyper::Client;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use tungstenite::WebSocket;
use tungstenite::stream::MaybeTlsStream;
use std::net::TcpStream;


/// # Основная структура библиотеки для одного аккаунта
pub struct API {
    /// `token` - Ваш token
    pub token:              String,

    /// * `broker_account_id` - ID, полученный из функции `get_accounts()`i
    pub broker_account_id:  String,
    http_client:            Client<HttpsConnector<HttpConnector>>,
    ws_client:              WebSocket<MaybeTlsStream<TcpStream>>
}
