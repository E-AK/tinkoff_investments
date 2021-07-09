use serde::{Serialize, Deserialize};


/// # Интервал свечей
/// Используется для выбора интервала свечей.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
/// * У каждого элемента есть атрибут со строкой, в которую он преобразуется
#[derive(Serialize, Deserialize, Debug)]
pub enum CandleResolution {
    /// `1min`
    #[serde(name="1min")]
    Min,

    /// `2min`
    #[serde(name="2min")]
    TwoMin,

    /// `3min`
    #[serde(name="3min")]
    ThreeMin,

    /// `5min`
    #[serde(name="5min")]
    FiveMin,

    /// `10min`
    #[serde(name="10min")]
    TenMin,

    /// `15min`
    #[serde(name="15min")]
    FifteenMin,

    /// `30min`
    #[serde(name="30min")]
    ThirtyMin,

    /// `hour`
    #[serde(name="hour")]
    Hour,

    /// `day`
    #[serde(name="day")]
    Day,

    /// `week`
    #[serde(name="week")]
    Week,

    /// `month`
    #[serde(name="month")]
    Month
}


/// # Структура ответа на запрос свечей
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в стуктуру
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct CandlesResponse {
    tracking_id:    str,

    /// Статус.
    status:         str,

    /// Свечи.
    payload:        Candles
}


/// # Структура свечей
/// Используется для хранения информации о свечах.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в стуктуру
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
struct Candles {
    /// Код инструмента.
    figi:       str,

    /// Интервал.
    interval:   CandleResolution,

    /// Массив свечей.
    candles:    [Candle]
}


/// # Структура свечи
/// Испольуется для хранения информации о свечи.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в стуктуру
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
pub struct Candle {
    /// Код инструмента.
    figi:       str,

    /// Интервал.
    interval:   CandleResolution,

    /// Цена открытия.
    o:          f32,

    /// Цена закрытия.
    c:          f32,

    /// Наивысшая цена.
    h:          f32,

    /// Наименьшая цена.
    l:          f32,

    /// Объем.
    v:          f32,

    /// Дата и время.
    time:       str
}
