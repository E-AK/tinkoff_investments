use std::collections::HashMap;

use hyper::{Request, Method, Body, Response, Error};

use super::params_str;


pub async fn orders(client: &super::Client, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::GET)
            .uri(format!("{}{}", super::ORDERS, params_str(params)))
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

pub async fn limit_order(client: &super::Client, req_body: String, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}{}", super::LIMIT_ORDER, params_str(params)))
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::from(req_body)).unwrap();

    client.hyper_client.request(req).await
}

pub async fn market_order(client: &super::Client, req_body: String, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}{}", super::MARKET_ORDER, params_str(params)))
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::from(req_body)).unwrap();

    client.hyper_client.request(req).await
}

pub async fn cancel(client: &super::Client, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}{}", super::CANCEL, params_str(params)))
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}