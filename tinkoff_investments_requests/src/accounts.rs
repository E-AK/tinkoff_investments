use hyper::{Method, Body, Response, Error};


pub async fn accounts(client: Box<super::Client>) -> Result<Response<Body>, Error> {
    let req = client.req
        .method(Method::GET)
        .uri(format!("{}{}", client.uri, "user/accounts"))
        .body(Body::empty()).unwrap();

    client.hyper_client.request(req).await
}
