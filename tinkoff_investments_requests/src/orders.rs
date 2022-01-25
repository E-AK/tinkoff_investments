use std::collections::HashMap;

use hyper::{Method, Body, Response, Error};

use super::params_str;


pub async fn orders(client: Box<super::Client>, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = client.req
        .method(Method::GET)
        .uri(format!("{}{}{}", client.uri, "orders", params_str(params)))
        .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

pub async fn limit_order(client: Box<super::Client>, req_body: String, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = client.req
        .method(Method::POST)
        .uri(format!("{}{}{}", client.uri, "orders/limit-order", params_str(params)))
        .body(Body::from(req_body)).unwrap();

    client.hyper_client.request(req).await
}

pub async fn market_order(client: Box<super::Client>, req_body: String, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = client.req
        .method(Method::POST)
        .uri(format!("{}{}{}", client.uri, "orders/market-order", params_str(params)))
        .body(Body::from(req_body)).unwrap();

    client.hyper_client.request(req).await
}

pub async fn cancel(client: Box<super::Client>, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = client.req
        .method(Method::POST)
        .uri(format!("{}{}{}", client.uri, "orders/cancel", params_str(params)))
        .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}