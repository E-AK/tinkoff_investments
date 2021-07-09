use serde::{Serialize, Deserialize};
use crate::schema::{Currency, InstrumentType};


/// # Структура ответа на запрос о списке инструментов
/// Используется для получения и хранения информации об инструментах.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct MarketInstrumentListResponse {
    tracking_id:    str,

    /// Статус.
    status:         str,

    /// Список инструментов.
    payload:        MarketInstrumentList
}


/// # Структура списка инструментов
/// Используется для храниния массива инструментов.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
struct MarketInstrumentList {
    total:          i32,

    /// Массив инструментов.
    instruments:    [MarketInstrument]
}


/// # Структура ответа на запрос: информация об инструменте
/// Используется для получения и хранения данных об инструменте.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
/// * Есть алиас с именем `SearchMarketInstrument`, т.к. есть структура с такими же полями
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
#[serde(alias="SearchMarketInstrumentResponse")]
struct MarketInstrumentResponse {
    tracking_id:    str,

    /// Статус.
    status:         str,

    /// Информация об инструменте.
    payload:        MarketInstrument
}


/// # Информация об инструменте
/// Используется для хранения информации об инструменте.
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
/// * Есть алиас с именем `SearchMarketInstrument`, т.к. есть структура с такими же полями
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
#[serde(alias="SearchMarketInstrument")]
struct MarketInstrument {
    /// Код инструмента.
    figi:                   str,

    ticker:                 str,
    isin:                   str,
    min_price_increment:    f32,

    /// Количество бумаг в лоте.
    lot:                    u32,
    min_quantity:           i32,

    /// Валюта.
    currency:               Currency,

    /// Название бумаги.
    name:                   str,

    /// Тип инструмента.
    r#type:                 InstrumentType
}
