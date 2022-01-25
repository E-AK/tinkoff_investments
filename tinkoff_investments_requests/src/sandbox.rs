use std::collections::HashMap;

use hyper::{Method, Body, Response, Error};

use super::params_str;


pub async fn register(client: Box<super::Client>, req_body: String) -> Result<Response<Body>, Error> {
    let req = client.req
        .method(Method::POST)
        .uri(format!("{}{}", client.uri, "sandbox/register"))
        .body(Body::from(req_body)).unwrap();

    client.hyper_client.request(req).await
}

pub async fn currencies_balance(client: Box<super::Client>, req_body: String, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = client.req
        .method(Method::POST)
        .uri(format!("{}{}{}", client.uri, "sandbox/currencies/balance", params_str(params)))
        .body(Body::from(req_body)).unwrap();

    client.hyper_client.request(req).await
}

pub async fn position_balance(client: Box<super::Client>, req_body: String, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = client.req
        .method(Method::POST)
        .uri(format!("{}{}{}", client.uri, "sandbox/positions/balance", params_str(params)))
        .body(Body::from(req_body)).unwrap();

    client.hyper_client.request(req).await
}

pub async fn remove(client: Box<super::Client>, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = client.req
        .method(Method::POST)
        .uri(format!("{}{}{}", client.uri, "sandbox/remove", params_str(params)))
        .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

pub async fn clear(client: Box<super::Client>, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = client.req
        .method(Method::POST)
        .uri(format!("{}{}{}", client.uri, "sandbox/clear", params_str(params)))
        .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}