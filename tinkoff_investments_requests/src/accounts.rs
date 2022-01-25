use hyper::{Request, Method, Body, Response, Error};


pub async fn accounts(client: &super::Client) -> Result<Response<Body>, Error> {
    let req = Request::builder()
            .method(Method::GET)
            .uri(format!("{}", super::ACCOUNTS))
            .header("accept", (String::from("application/json")).as_str())
            .header("Content-Type", (String::from("application/json")).as_str())
            .header("Authorization", (String::from("Bearer ") + &client.token).as_str())
            .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}