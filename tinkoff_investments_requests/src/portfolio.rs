use std::collections::HashMap;

use hyper::{Request, Method, Body, Response, Error};

use super::params_str;


pub async fn portfolio(client: &super::Client, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::GET)
            .uri(format!("{}{}", super::PORTFOLIO, params_str(params)))
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

pub async fn currencies(client: &super::Client, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::GET)
            .uri(format!("{}{}", super::PORTFOLIO_CURRENCIES, params_str(params)))
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}