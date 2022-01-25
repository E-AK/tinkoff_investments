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
    async fn remove() {
        let client = Box::new(super::create_client());
        let resp = sandbox::remove(client, None).await;

        match resp {
            Ok(ok) => assert_eq!(ok.status(), hyper::StatusCode::from_u16(200).unwrap()),
            Err(_) => panic!("Ошибка соединения")
        }
    }
}