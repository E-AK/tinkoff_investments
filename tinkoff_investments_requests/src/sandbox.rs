use std::collections::HashMap;

use hyper::{Request, Method, Body, Response, Error};

use super::params_str;


pub async fn register(client: &super::Client, req_body: String) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}", super::SANDBOX_REGISTER))
            .header("accept", (String::from("application/json")).as_str())
            .header("Content-Type", (String::from("application/json")).as_str())
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::from(req_body)).unwrap();

    client.hyper_client.request(req).await
}

pub async fn currencies_balance(client: &super::Client, req_body: String, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}{}", super::SANDBOX_CURRENCIES_BALANCE, params_str(params)))
            .header("accept", (String::from("application/json")).as_str())
            .header("Content-Type", (String::from("application/json")).as_str())
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::from(req_body)).unwrap();

    client.hyper_client.request(req).await
}

pub async fn position_balance(client: &super::Client, req_body: String, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}{}", super::SANDBOX_CURRENCIES_BALANCE, params_str(params)))
            .header("accept", (String::from("application/json")).as_str())
            .header("Content-Type", (String::from("application/json")).as_str())
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::from(req_body)).unwrap();

    client.hyper_client.request(req).await
}

pub async fn remove(client: &super::Client, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}{}", super::SANDBOX_REMOVE, params_str(params)))
            .header("accept", (String::from("application/json")).as_str())
            .header("Content-Type", (String::from("application/json")).as_str())
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

pub async fn clear(client: &super::Client, params: Option<HashMap<String, String>>) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}{}", super::SANDBOX_CLEAR, params_str(params)))
            .header("accept", (String::from("application/json")).as_str())
            .header("Content-Type", (String::from("application/json")).as_str())
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}

#[cfg(test)]
mod all {
    use core::panic;
    use std::env;
    use hyper_tls::HttpsConnector;
    use crate::Client;

    fn create_client() -> crate::Client {
        let token = env::var("TINKOFF_SANDBOX_TOKEN").expect("Нет ключа");
        let https = HttpsConnector::new();
        let hyper_client = hyper::Client::builder()
            .build::<_, hyper::Body>(https);

        Client {
            token,
            hyper_client
        }
    }

    #[tokio::test]
    async fn sandbox_register() {
        let client = create_client();
        let resp = super::register(&client, String::from("{\"brokerAccountType\": \"Tinkoff\"}")).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(200).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }

    #[tokio::test]
    async fn sandbox_register_error() {
        let client = create_client();
        let resp = super::register(&client, String::from("{\"brokerAccountType\": \"Tinkofff\"}")).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(500).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }
}