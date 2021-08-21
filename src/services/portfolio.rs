use crate::API;
use crate::services::{BASE_URI, PORTFOLIO, PORTFOLIO_CURRENCIES};

use hyper::{Method, Body};
use std::collections::HashMap;
use crate::schemas::portfolio::{Portfolio};
use crate::schemas::error::Error;
use crate::schemas::Resp;
use crate::schemas::currency::Currencies;


impl API {
    /// # Получение портфеля
    pub async fn get_portfolio(&self) -> Result<Resp<Portfolio>, Resp<Error>> {
        let mut params = HashMap::new();

        params.insert("brokerAccountId".to_string(), self.broker_account_id.clone());

        let data = self.req(
            BASE_URI.to_string(),
            PORTFOLIO.to_string(),
            Method::GET,
            params,
            Body::empty()).await;

        return match data {
            Ok(obj) =>
                Ok(serde_json::from_str::<Resp<Portfolio>>(&obj).unwrap()),
            Err(err) => Err(err)
        }
    }

    /// # Получение валютных активов
    pub async fn get_portfolio_currencies(&self) -> Result<Resp<Currencies>, Resp<Error>> {
        let mut params = HashMap::new();

        params.insert("brokerAccountId".to_string(), self.broker_account_id.clone());

        let data = self.req(
            BASE_URI.to_string(),
            PORTFOLIO_CURRENCIES.to_string(),
            Method::GET,
            params,
            Body::empty()).await;

        return match data {
            Ok(obj) =>
                Ok(serde_json::from_str::<Resp<Currencies>>(&obj).unwrap()),
            Err(err) => Err(err)
        }
    }
}