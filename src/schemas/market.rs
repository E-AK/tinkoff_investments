use crate::schemas::{Currency, InstrumentType, TradeStatus, Resp};

use serde::{Serialize, Deserialize};
use crate::schemas::order::Order;


/// # Структура списка инструментов
/// Используется для храниния массива инструментов.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
pub struct MarketInstrumentList {
    pub total:          i32,

    /// Массив инструментов.
    pub instruments:    Vec<MarketInstrument>
}


/// # Информация об инструменте
/// Используется для хранения информации об инструменте.
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
/// * Есть алиас с именем `SearchMarketInstrument`, т.к. есть структура с такими же полями
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct MarketInstrument {
    /// Код инструмента.
    pub figi:                   String,

    pub ticker:                 String,

    #[serde(default)]
    pub isin:                   String,

    #[serde(default)]
    pub min_price_increment:    f32,

    /// Количество бумаг в лоте.
    pub lot:                    u32,

    #[serde(default)]
    pub min_quantity:           u32,

    /// Валюта.
    pub currency:               Currency,

    /// Название бумаги.
    pub name:                   String,

    /// Тип инструмента.
    pub r#type:                 InstrumentType
}


/// Структура стакана.
/// Используется для хранения стакана.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct OrderBook {
    /// Код инструмента.
    pub figi:                   String,

    /// Глубина.
    pub depth:                  i32,
    pub bids:                   Vec<Resp<Order>>,
    pub asks:                   Vec<Resp<Order>>,

    /// Статус торгов.
    pub trade_status:           TradeStatus,
    pub min_price_increment:    f32,

    #[serde(default)]
    pub face_value:             f32,
    pub last_price:             f32,
    pub close_price:            f32,
    pub limit_up:               f32,
    pub limit_down:             f32
}
