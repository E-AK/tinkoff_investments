use serde::{Serialize, Deserialize};
use crate::schema::{InstrumentType, MoneyAmount};
use crate::schema::currency::Currencies;


/// # Структура ответа на запрос портфеля
/// Используется для получения и хранения информации о портфеле.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct PortfolioResponse {
    tracking_id:    str,

    /// Портфель.
    status:         str,

    /// Портфель.
    payload:        Portfolio,
}


/// # Структура портфеля
/// Используется для хранения иформации о портфеле.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
struct Portfolio {
    /// Массив позиций в порфеле.
    positions: [PortfolioPosition],
}


/// # Структура ответа на запрос валюты в порфеле
/// Используется для получения и хранения информации о валюте в порфеле.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct PortfolioCurrenciesResponse {
    tracking_id:    str,

    /// Статус.
    status:         str,

    /// Валюта.
    payload:        Currencies
}


/// # Структура позиции в портфеле
/// Используется для хранения информации о позиции.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct PortfolioPosition {
    /// Код инструмента.
    figi:                           str,
    ticker:                         str,
    isin:                           str,

    /// Тип инструмента.
    instrument_type:                InstrumentType,

    /// Баланс.
    balance:                        f32,

    /// Заблокировано средств.
    blocked:                        f32,
    expected_yield:                 MoneyAmount,

    /// Количество лотов.
    lots:                           u32,
    average_position_price:         MoneyAmount,
    average_position_price_no_nkg:  MoneyAmount,

    /// Имя.
    name:                           str,
}
