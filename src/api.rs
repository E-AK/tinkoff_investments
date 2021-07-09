mod sandbox;
mod orders;

use hyper::{Client, Method, Request, Body, StatusCode};
use hyper::client::HttpConnector;
use hyper::body::Bytes;
use bytes::{BytesMut, BufMut};
use crate::schema::error::Error;


const BASE_URI: String = String::from("https://api-invest.tinkoff.ru/openapi/");
pub const SANDBOX_URI: &str = "https://api-invest.tinkoff.ru/openapi/sandbox/";
const STREAMING_URI: String = String::from("wss://api-invest.tinkoff.ru/openapi/md/v1/md-openapi/ws");

pub const SANDBOX_REGISTER: &str = "register";
pub const SANDBOX_CURRENCIES_BALANCE: String = String::from("currencies/balance");
pub const SANDBOX_POSITIONS_BALANCE: String = String::from("positions/balance");
pub const SANDBOX_REMOVE: String = String::from("remove");
pub const SANDBOX_CLEAR: String = String::from("clear");

const ORDERS: String = String::from("orders");
const LIMIT_ORDER: String = String::from("orders/limit-order");
const MARKET_ORDER: String = String::from("orders/market-order");
const CANCEL: String = String::from("orders/cancel");

const PORTFOLIO: String = String::from("portfolio");
const PORTFOLIO_CURRENCIES: String = String::from("portfolio/currencies");

const STOCKS: String = String::from("market/stocks");
const BONDS: String = String::from("market/bonds");
const ETFS: String = String::from("market/etfs");
const CURRENCIES: String = String::from("market/currencies");
const ORDER_BOOK: String = String::from("market/orderbook");
const CANDLES: String = String::from("market/candles");
const BY_FIGI: String = String::from("market/by-figi");
const BY_TICKER: String = String::from("market/by-ticker");

const OPERATIONS: String = String::from("operations");
const USER: String = String::from("user/account");


pub struct API {
    token: String,
    http_client: Client<HttpConnector>,
}

impl API {
    pub async fn req(&self, uri: &str, path: &str, method: Method, req_body: Body)
        -> Result<BytesMut, Error> {
        let req = Request::builder()
            .method(method)
            .uri(uri + path)
            .header("Authorization", "Bearer " + &self.token)
            .body(req_body)?;

        let mut resp = self.http_client.request(req).await?;

        let mut resp_body = BytesMut::with_capacity(1024);
        let status = resp.status().as_u16();

        while let Some(chunk) = resp.body_mut().data().await {
            req_body.put(chunk?);
        }

        match status {
            500 => {
                let error = serde_json::from_slice::<Error>(resp_body.as_ref())?;
                println!("{:?}", error);
                Err(error)
            },
            200 => Ok(resp_body),
            _ => panic!("Неизвестная ошибка")
        }
    }
}