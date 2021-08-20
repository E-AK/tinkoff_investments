use crate::Service;
use crate::schemas::error::Error;
use crate::services::{BASE_URI, OPERATIONS, dt_fmt};

use std::collections::HashMap;
use hyper::{Method, Body};
use chrono::{DateTime, Utc};
use crate::schemas::Resp;
use crate::schemas::operation::Operations;


impl Service {
    /// # Получение списка операций
    /// * `from` - Начало временного промежутка
    /// * `to` - Конец временного промежутка
    /// * `figi` - Код инструмента для фильтрации
    pub async fn get_operations(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        figi: String) -> Result<Resp<Operations>, Resp<Error>>
    {
        let mut params = HashMap::new();

        params.insert("from".to_string(), dt_fmt(from));
        params.insert("to".to_string(), dt_fmt(to));
        params.insert("figi".to_string(), figi);
        params.insert("brokerAccountId".to_string(), self.broker_account_id.clone());

        let data = self.req(
            BASE_URI.to_string(),
            OPERATIONS.to_string(),
            Method::GET,
            params,
            Body::empty()).await;

        return match data {
            Ok(obj) =>
                Ok(serde_json::from_str::<Resp<Operations>>(&obj).unwrap()),
            Err(err) => Err(err)
        }
    }
}