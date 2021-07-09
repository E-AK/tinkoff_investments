use serde::{Serialize, Deserialize};
use crate::schema::currency::Currency;
use crate::schema::account::{BrokerAccountType, UserAccount};


/// # Структура запроса регистрации песочницы
/// Используется для созтавления запроса регистрации песочницы.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct SandboxRegisterRequest {
    broker_account_type: BrokerAccountType
}


/// # Структура ответа о регистрации песочницы
/// Используется для получения и зранения информации об аккаунте в песочнице.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct SandboxRegisterResponse {
    tracking_id:    str,

    /// Статус.
    status:         str,

    /// Аккаунт.
    payload:        UserAccount
}


/// # Структура запроса установки баланса в песочнице
/// Используется для составления запроса установки баланса в песочнице.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
pub struct SandboxSetCurrencyBalanceRequest {
    /// Валюта.
    currency:   Currency,

    /// Баланс.
    balance:    f32
}


/// # Структура запроса установки позиции в песочнице
/// Используется для составления запроса установки позиции в песочнице.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
pub struct SandboxSetPositionBalanceRequest {
    /// Код инструмента.
    figi:       String,

    /// Баланс.
    balance:    f32
}
