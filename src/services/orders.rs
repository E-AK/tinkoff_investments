use crate::Service;
use crate::services::{BASE_URI, ORDERS, LIMIT_ORDER, MARKET_ORDER, CANCEL};
use crate::schemas::order::{LimitOrderRequest, MarketOrderRequest, Order, PlacedLimitOrder, PlacedMarketOrder};

use std::collections::HashMap;
use hyper::{Method, Body};
use crate::schemas::{Resp};
use crate::schemas::error::Error;


impl Service {
    /// # Получение списка активных заявок
    pub async fn get_orders(&self) -> Result<Resp<Vec<Order>>, Resp<Error>> {
        let mut params = HashMap::new();

        params.insert("brokerAccountId".to_string(), self.broker_account_id.clone());

        let data = self.req(
            BASE_URI.to_string(),
            ORDERS.to_string(),
            Method::GET,
            params,
            Body::empty()).await;

        return match data {
            Ok(obj) =>
                Ok(serde_json::from_str::<Resp<Vec<Order>>>(&obj).unwrap()),
            Err(err) => Err(err)
        }
    }

    /// # Создание лимитной заявки
    /// * `req` - Структура запроса
    /// * `figi` - Код инструмента
    pub async fn limit_order(&self, req: LimitOrderRequest, figi: String) -> Result<Resp<PlacedLimitOrder>, Resp<Error>> {
        let body = serde_json::to_string(&req).unwrap();

        let mut params = HashMap::new();

        params.insert("figi".to_string(), figi);
        params.insert("brokerAccountId".to_string(), self.broker_account_id.clone());

        let data = self.req(
            BASE_URI.to_string(),
            LIMIT_ORDER.to_string(),
            Method::POST,
            params,
            Body::from(body)).await;

        return match data {
            Ok(obj) =>
                Ok(serde_json::from_str::<Resp<PlacedLimitOrder>>(&obj).unwrap()),
            Err(err) => Err(err)
        }
    }

    /// # Создание рыночной заявки
    /// * `req` - Структура запроса
    /// * `figi` - Код инструмента
    pub async fn market_order(&self, req: MarketOrderRequest, figi: String) -> Result<Resp<PlacedMarketOrder>, Resp<Error>> {
        let body = serde_json::to_string(&req).unwrap();

        let mut params = HashMap::new();

        params.insert("figi".to_string(), figi);
        params.insert("brokerAccountId".to_string(), self.broker_account_id.clone());

        let data = self.req(
            BASE_URI.to_string(),
            MARKET_ORDER.to_string(),
            Method::POST,
            params,
            Body::from(body)).await;

        return match data {
            Ok(obj) =>
                Ok(serde_json::from_str::<Resp<PlacedMarketOrder>>(&obj).unwrap()),
            Err(err) => Err(err)
        }
    }

    /// # Отмена заявки
    /// * `id` - ID заявки
    pub async fn cancel(&self, id: String) -> Result<Resp<()>, Resp<Error>> {
        let mut params = HashMap::new();

        params.insert("orderId".to_string(), id);
        params.insert("brokerAccountId".to_string(), self.broker_account_id.clone());

        let data = self.req(
            BASE_URI.to_string(),
            CANCEL.to_string(),
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