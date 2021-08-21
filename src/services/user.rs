use crate::API;
use crate::services::{BASE_URI, USER};
use crate::schemas::account::UserAccounts;

use std::collections::HashMap;
use hyper::{Body, Method};
use crate::schemas::error::Error;
use crate::schemas::Resp;

impl API {
    /// # Получение брокерских счетов
    pub async fn get_accounts(&self) -> Result<Resp<UserAccounts>, Resp<Error>> {
        let data = self.req(
            BASE_URI.to_string(),
            USER.to_string(),
            Method::GET,
            HashMap::new(),
            Body::empty()).await;

        return match data {
            Ok(obj) =>
                Ok(serde_json::from_str::<Resp<UserAccounts>>(&obj).unwrap()),
            Err(err) => Err(err)
        }
    }
}