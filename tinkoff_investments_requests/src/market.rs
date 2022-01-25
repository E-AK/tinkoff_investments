use std::collections::HashMap;

use hyper::{Request, Method, Body, Response, Error};

use super::params_str;


pub async fn stocks(client: &super::Client) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::GET)
            .uri(format!("{}", super::STOCKS))
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

pub async fn bonds(client: &super::Client) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::GET)
            .uri(format!("{}", super::BONDS))
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

pub async fn etfs(client: &super::Client) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::GET)
            .uri(format!("{}", super::ETFS))
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

pub async fn currencies(client: &super::Client) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::GET)
            .uri(format!("{}", super::CURRENCIES))
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

pub async fn orderbook(client: &super::Client, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::GET)
            .uri(format!("{}{}", super::ORDERS, params_str(params)))
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

pub async fn candles(client: &super::Client, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::GET)
            .uri(format!("{}{}", super::CANDLES, params_str(params)))
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

pub async fn by_figi(client: &super::Client, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::GET)
            .uri(format!("{}{}", super::BY_FIGI, params_str(params)))
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

pub async fn by_ticker(client: &super::Client, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::GET)
            .uri(format!("{}{}", super::BY_TICKER, params_str(params)))
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}