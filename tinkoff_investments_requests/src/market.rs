use std::collections::HashMap;

use hyper::{Method, Body, Response, Error};

use super::params_str;


pub async fn stocks(client: Box<super::Client>) -> Result<Response<Body>, Error> {
    let req = client.req
        .method(Method::GET)
        .uri(format!("{}{}", client.uri, "market/stocks"))
        .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

pub async fn bonds(client: Box<super::Client>) -> Result<Response<Body>, Error> {
    let req = client.req
        .method(Method::GET)
        .uri(format!("{}{}", client.uri, "market/bonds"))
        .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

pub async fn etfs(client: Box<super::Client>) -> Result<Response<Body>, Error> {
    let req = client.req
        .method(Method::GET)
        .uri(format!("{}{}", client.uri, "market/etfs"))
        .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

pub async fn currencies(client: Box<super::Client>) -> Result<Response<Body>, Error> {
    let req = client.req
        .method(Method::GET)
        .uri(format!("{}{}", client.uri, "market/currencies"))
        .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

pub async fn orderbook(client: Box<super::Client>, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = client.req
        .method(Method::GET)
        .uri(format!("{}{}{}", client.uri, "market/orderbook", params_str(params)))
        .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

pub async fn candles(client: Box<super::Client>, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = client.req
        .method(Method::GET)
        .uri(format!("{}{}{}", client.uri, "market/candles", params_str(params)))
        .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

pub async fn by_figi(client: Box<super::Client>, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = client.req
        .method(Method::GET)
        .uri(format!("{}{}{}", client.uri, "search/by-ticker", params_str(params)))
        .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

pub async fn by_ticker(client: Box<super::Client>, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = client.req
        .method(Method::GET)
        .uri(format!("{}{}{}", client.uri, "search/by-ticker", params_str(params)))
        .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}