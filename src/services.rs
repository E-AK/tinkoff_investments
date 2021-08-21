pub mod sandbox;
mod orders;
pub mod portfolio;
pub mod market;
mod operations;
mod user;


use crate::schemas::error::Error;
use crate::API;
use crate::schemas::Resp;

use std::collections::HashMap;
use hyper::{Client, Method, Request, Body};
use hyper_tls::HttpsConnector;
use chrono::{DateTime, Utc, Datelike, Timelike};
use tungstenite::connect;
use url::Url;


pub const BASE_URI: &str = "https://api-invest.tinkoff.ru/openapi/";
pub const SANDBOX_URI: &str = "https://api-invest.tinkoff.ru/openapi/sandbox/";
pub const STREAMING_URI: &str = "wss://api-invest.tinkoff.ru/openapi/md/v1/md-openapi/ws";

pub const SANDBOX_REGISTER: &str = "register";
pub const SANDBOX_CURRENCIES_BALANCE: &str = "currencies/balance";
pub const SANDBOX_POSITIONS_BALANCE: &str = "positions/balance";
pub const SANDBOX_REMOVE: &str = "remove";
pub const SANDBOX_CLEAR: &str = "clear";

pub const ORDERS: &str = "orders";
pub const LIMIT_ORDER: &str = "orders/limit-order";
pub const MARKET_ORDER: &str = "orders/market-order";
pub const CANCEL: &str = "orders/cancel";

pub const PORTFOLIO: &str = "portfolio";
pub const PORTFOLIO_CURRENCIES: &str = "portfolio/currencies";

pub const STOCKS: &str = "market/stocks";
pub const BONDS: &str = "market/bonds";
pub const ETFS: &str = "market/etfs";
pub const CURRENCIES: &str = "market/currencies";
pub const ORDER_BOOK: &str = "market/orderbook";
pub const CANDLES: &str = "market/candles";
pub const BY_FIGI: &str = "market/search/by-figi";
pub const BY_TICKER: &str = "market/search/by-ticker";

pub const OPERATIONS: &str = "operations";
pub const USER: &str = "user/accounts";

impl API {
    pub fn new(token: String) -> API {
        let https = HttpsConnector::new();
        let http_client = Client::builder()
            .build::<_, hyper::Body>(https);
        let (ws_client, _) =
            connect(Url::parse(&STREAMING_URI).unwrap()).expect("Can't connect.");

        API {
            token,
            broker_account_id: "".to_string(),
            http_client,
            ws_client
        }
    }

    pub async fn req(&self,
                     uri: String,
                     path: String,
                     method: Method,
                     params: HashMap<String, String>,
                     req_body: Body)
                     -> Result<String, Resp<Error>>
    {
        let mut params_str = String::from("?");

        for (k, v) in params {

            let param = format!("{}={}&", k, v);
            params_str.push_str(&param);
        }

        let req = Request::builder()
            .method(method)
            .uri(format!("{}{}{}", uri, path, params_str))
            .header("Authorization", (String::from("Bearer ") + &self.token).as_str())
            .body(req_body).unwrap();

        let resp = self.http_client.request(req).await.unwrap();
        let status = resp.status();
        let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
        let data = String::from_utf8(body.to_vec()).unwrap();

        return match status.as_u16() {
            200 => Ok(data),
            500 => Err(serde_json::from_str::<Resp<Error>>(&data).unwrap()),
            _ => panic!("Неизвестная ошибка: {}", data)
        }
    }
}

pub fn dt_fmt(dt: DateTime<Utc>) -> String{
    format!("{}-{:02}-{:02}T{:02}%3A{:02}%3A{:02}%2B03%3A00",
            dt.year(), dt.month(), dt.day(), dt.hour(), dt.minute(), dt.second())
}