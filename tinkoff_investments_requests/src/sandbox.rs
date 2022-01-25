use std::collections::HashMap;

use hyper::{Request, Method, Body, Response, Error};

use super::params_str;


pub async fn register(client: &super::Client, req_body: String) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}", super::SANDBOX_REGISTER))
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::from(req_body)).unwrap();

    client.hyper_client.request(req).await
}

pub async fn currencies_balance(client: &super::Client, req_body: String, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}{}", super::SANDBOX_CURRENCIES_BALANCE, params_str(params)))
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::from(req_body)).unwrap();

    client.hyper_client.request(req).await
}

pub async fn position_balance(client: &super::Client, req_body: String, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}{}", super::SANDBOX_CURRENCIES_BALANCE, params_str(params)))
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::from(req_body)).unwrap();

    client.hyper_client.request(req).await
}

pub async fn remove(client: &super::Client, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}{}", super::SANDBOX_REMOVE, params_str(params)))
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

pub async fn clear(client: &super::Client, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}{}", super::SANDBOX_CLEAR, params_str(params)))
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}