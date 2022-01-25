use std::env;

use tinkoff_investments_requests::{Client, SANDBOX_URI};


fn create_client() -> Client {
    let token = env::var("TINKOFF_SANDBOX_TOKEN").expect("Нет ключа");
    
    Client::new(token, SANDBOX_URI)
}


#[cfg(test)]
mod accounts {
    use tinkoff_investments_requests::accounts;

    #[tokio::test]
    async fn accounts() {
        let client = Box::new(super::create_client());
        let resp = accounts::accounts(client).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(200).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }
}


#[cfg(test)]
mod market {
    use std::collections::HashMap;

    use tinkoff_investments_requests::market;

    #[tokio::test]
    async fn stocks() {
        let client = Box::new(super::create_client());
        let resp = market::stocks(client).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(200).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }

    #[tokio::test]
    async fn bonds() {
        let client = Box::new(super::create_client());
        let resp = market::bonds(client).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(200).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }

    #[tokio::test]
    async fn etfs() {
        let client = Box::new(super::create_client());
        let resp = market::etfs(client).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(200).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }

    #[tokio::test]
    async fn currencies() {
        let client = Box::new(super::create_client());
        let resp = market::currencies(client).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(200).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }

    #[tokio::test]
    async fn orderbook() {
        let client = Box::new(super::create_client());
        let resp = market::orderbook(client, None).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(500).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }

    #[tokio::test]
    async fn candles() {
        let client = Box::new(super::create_client());
        let resp = market::candles(client, None).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(500).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }

    #[tokio::test]
    async fn by_figi() {
        let client = Box::new(super::create_client());
        let resp = market::by_figi(client, None).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(500).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }

    #[tokio::test]
    async fn by_ticker() {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("ticker"), String::from("SBER"));

        let client = Box::new(super::create_client());
        let resp = market::by_ticker(client, Some(params)).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(200).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }
}


#[cfg(test)]
mod operations{
    use tinkoff_investments_requests::operations;

    #[tokio::test]
    async fn by_figi() {
        let client = Box::new(super::create_client());
        let resp = operations::operations(client, None).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(500).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }
}


#[cfg(test)]
mod orders{
    use tinkoff_investments_requests::orders;

    #[tokio::test]
    async fn orders() {
        let client = Box::new(super::create_client());
        let resp = orders::orders(client, None).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(200).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }
}


#[cfg(test)]
mod sandbox {
    use core::panic;
    use tinkoff_investments_requests::sandbox;

    #[tokio::test]
    async fn register() {
        let client = Box::new(super::create_client());
        let resp = sandbox::register(client, String::from("{\"brokerAccountType\": \"Tinkoff\"}")).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(200).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }


    #[tokio::test]
    async fn currencies_balance() {
        let client = Box::new(super::create_client());
        let resp = sandbox::currencies_balance(client, String::from("{\"currency\": \"RUB\",\"balance\": 0}"), None).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(200).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }

    #[tokio::test]
    async fn position_balance() {
        let client = Box::new(super::create_client());
        let resp = sandbox::position_balance(client, String::from("{\"figi\": \"string\",\"balance\": 0}"), None).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(200).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }

    #[tokio::test]
    async fn remove() {
        let client = Box::new(super::create_client());
        let resp = sandbox::remove(client, None).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(200).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }

    #[tokio::test]
    async fn clear() {
        let client = Box::new(super::create_client());
        let resp = sandbox::clear(client, None).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(200).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }
}