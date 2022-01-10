pub mod account;
pub mod order;
pub mod market;
pub mod operation;
pub mod currency;
pub mod error;
pub mod candle;
pub mod sandbox;
pub mod streaming;
pub mod portfolio;


use crate::schemas::currency::Currency;

use serde::{Serialize, Deserialize};


enum RequestTypes {
    LimitOrderRequest(order::LimitOrderRequest),
    MarketOrderRequest(order::MarketOrderRequest),
    SandboxRegisterRequest(sandbox::SandboxRegisterRequest),
    SandboxSetCurrencyBalanceRequest(sandbox::SandboxSetCurrencyBalanceRequest),
    SandboxSetPositionBalanceRequest(sandbox::SandboxSetPositionBalanceRequest),
}

enum ResponseTypes {
    Error(error::Error),
    SandboxRegisterResponse(sandbox::SandboxRegisterResponse),
    OrdersResponse(order::OrdersResponse)
}

/// # Структура ответа на запрос
/// Используется для получения и хранения информации о брокерских счетах.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в стуктуру
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Resp<TPayload> {
    pub tracking_id:    String,
    pub status:         String,
    pub payload:        TPayload
}


/// # Структура информации о средствах.
/// Используется для указания средств.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в стуктуру
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
pub struct MoneyAmount {
    /// Валюта.
    pub currency: Currency,

    /// Значение.
    pub value: f32,
}


/// # Статус торгов
/// Используется для указания статуса торгов.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в стуктуру
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
pub enum TradeStatus {
    /// Нормальная торговля.
    NormalTrading,

    /// Недоступно для торговли.
    NotAvailableForTrading,
}


/// # Тип инструмента
/// Используется для выбора типа бумаги.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в стуктуру
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
pub enum InstrumentType {
    /// Акции.
    Stock,

    /// Валюта.
    Currency,

    /// Облигации.
    Bond,

    /// Фонды.
    Etf,
}
