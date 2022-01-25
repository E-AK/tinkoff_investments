#[cfg(test)]
mod all {
    use core::panic;
    use std::env;
    use hyper_tls::HttpsConnector;
    use tinkoff_investments_requests::{Client, sandbox};

    fn create_client() -> Client {
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
        let resp = sandbox::register(&client, String::from("{\"brokerAccountType\": \"Tinkoff\"}")).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(200).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }

    #[tokio::test]
    async fn sandbox_register_error() {
        let client = create_client();
        let resp = sandbox::register(&client, String::from("{\"brokerAccountType\": \"Tinkofff\"}")).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(500).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }
}