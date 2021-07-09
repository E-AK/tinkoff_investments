use crate::schema::currency::Currency;
use serde::{Serialize, Deserialize};

mod portfolio;
pub mod account;
pub mod order;
mod market;
mod operation;
pub mod currency;
pub mod error;
mod candle;
pub mod sandbox;
mod streaming;


/// # Безымянная структура
/// Используется для получения и хранения неопределенных данных.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в стуктуру
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Empty<TPayload> {
    tracking_id: str,

    /// Какие-то данные.
    payload: TPayload,

    /// Статус.
    status: str,
}


/// # Структура информации о средствах.
/// Используется для указания средств.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в стуктуру
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
struct MoneyAmount {
    /// Валюта.
    currency: Currency,

    /// Значение.
    value: f32,
}


/// # Статус торгов
/// Используется для указания статуса торгов.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в стуктуру
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
enum TradeStatus {
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
