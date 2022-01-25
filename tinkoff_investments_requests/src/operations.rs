use std::collections::HashMap;

use hyper::{Method, Body, Response, Error};

use super::params_str;


pub async fn operations(client: Box<super::Client>, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = client.req
        .method(Method::GET)
        .uri(format!("{}{}{}", client.uri, "operations", params_str(params)))
        .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}