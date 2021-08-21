use crate::schemas::currency::Currency;
use crate::schemas::account::{BrokerAccountType};

use serde::{Serialize, Deserialize};


/// # Структура запроса регистрации песочницы
/// Используется для созтавления запроса регистрации песочницы.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct SandboxRegisterRequest {
    pub broker_account_type: BrokerAccountType
}


/// # Структура запроса установки баланса в песочнице
/// Используется для составления запроса установки баланса в песочнице.
#[derive(Serialize, Deserialize, Debug)]
pub struct SandboxSetCurrencyBalanceRequest {
    /// Валюта.
    pub currency:   Currency,

    /// Баланс.
    pub balance:    f32
}


/// # Структура запроса установки позиции в песочнице
/// Используется для составления запроса установки позиции в песочнице.
#[derive(Serialize, Deserialize, Debug)]
pub struct SandboxSetPositionBalanceRequest {
    /// Код инструмента.
    pub figi:       String,

    /// Баланс.
    pub balance:    f32
}
