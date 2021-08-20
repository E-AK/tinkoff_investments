use crate::Service;
use crate::schemas::sandbox::{SandboxRegisterRequest, SandboxSetCurrencyBalanceRequest,
                              SandboxSetPositionBalanceRequest};
use crate::services::{SANDBOX_URI, SANDBOX_REGISTER, SANDBOX_CURRENCIES_BALANCE,
                      SANDBOX_POSITIONS_BALANCE, SANDBOX_REMOVE, SANDBOX_CLEAR};
use crate::schemas::{Resp};

use hyper::{Method, Body};
use std::collections::HashMap;
use crate::schemas::error::Error;
use crate::schemas::account::UserAccount;


impl Service {
    /// # Регистрация клиента в sandbox
    /// * `req` - Структура запроса
    pub async fn reg(&self, req: SandboxRegisterRequest) -> Result<Resp<UserAccount>, Resp<Error>> {
        let body = serde_json::to_string(&req).unwrap();

        let data = self.req(
            SANDBOX_URI.to_string(),
            SANDBOX_REGISTER.to_string(),
            Method::POST,
            HashMap::new(),
            Body::from(body)).await;

        return match data {
            Ok(obj) =>
                Ok(serde_json::from_str::<Resp<UserAccount>>(&obj).unwrap()),
            Err(err) => Err(err)
        }
    }

    /// # Выставление баланса по валютным позициям
    /// * `req` - Структура запроса
    pub async fn currency_balance(&self, req: SandboxSetCurrencyBalanceRequest) -> Result<Resp<()>, Resp<Error>> {
        let body = serde_json::to_string(&req).unwrap();

        let mut params = HashMap::new();

        params.insert("brokerAccountId".to_string(), self.broker_account_id.clone());

        let data = self.req(
            SANDBOX_URI.to_string(),
            SANDBOX_CURRENCIES_BALANCE.to_string(),
            Method::POST,
            params,
            Body::from(body)).await;

        return match data {
            Ok(obj) =>
                Ok(serde_json::from_str::<Resp<()>>(&obj).unwrap()),
            Err(err) => Err(err)
        }
    }

    /// # Выставление баланса по инструментным позициям
    /// * `req` - Структура запроса
    pub async fn position_balance(&self, req: SandboxSetPositionBalanceRequest) -> Result<Resp<()>, Resp<Error>> {
        let body = serde_json::to_string(&req).unwrap();

        let mut params = HashMap::new();

        params.insert("brokerAccountId".to_string(), self.broker_account_id.clone());

        let data = self.req(
            SANDBOX_URI.to_string(),
            SANDBOX_POSITIONS_BALANCE.to_string(),
            Method::POST,
            params,
            Body::from(body)).await;

        return match data {
            Ok(obj) =>
                Ok(serde_json::from_str::<Resp<()>>(&obj).unwrap()),
            Err(err) => Err(err)
        }
    }

    /// # Удаление счета
    pub async fn remove(&mut self) -> Result<Resp<()>, Resp<Error>> {
        let mut params = HashMap::new();

        params.insert("brokerAccountId".to_string(), self.broker_account_id.clone());
        self.broker_account_id = "".to_string();

        let data = self.req(
            SANDBOX_URI.to_string(),
            SANDBOX_REMOVE.to_string(),
            Method::POST,
            params,
            Body::empty()).await;

        return match data {
            Ok(obj) =>
                Ok(serde_json::from_str::<Resp<()>>(&obj).unwrap()),
            Err(err) => Err(err)
        }
    }

    // # Удаление всех позиций
    pub async fn clear(&self) -> Result<Resp<()>, Resp<Error>> {
        let mut params = HashMap::new();

        params.insert("brokerAccountId".to_string(), self.broker_account_id.clone());

        let data = self.req(
            SANDBOX_URI.to_string(),
            SANDBOX_CLEAR.to_string(),
            Method::POST,
            params,
            Body::empty()).await;

        return match data {
            Ok(obj) =>
                Ok(serde_json::from_str::<Resp<()>>(&obj).unwrap()),
            Err(err) => Err(err)
        }
    }
}