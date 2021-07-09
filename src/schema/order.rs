use serde::{Serialize, Deserialize};
use crate::schema::{TradeStatus, MoneyAmount};
use crate::schema::operation::OperationType;


/// # Статус заявки
/// Используется для отображения статуса заявки.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
enum OrderStatus {
    /// Новая.
    New,

    /// Частично заполнена.
    PartiallyFill,

    /// Заполнена.
    Fill,

    /// Отменена.
    Cancelled,

    /// Заменена.
    Replaced,

    /// Ожидает отмены.
    PendingCancel,

    /// Откланена.
    Rejected,

    /// Ожидает замены.
    PendingReplace,

    /// В ожидании новой.
    PendingNew
}


/// # Тип заявки
/// Используется для указания типа заявки.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
enum OrderType {
    /// Лимитная.
    Limit,

    /// Рыночная.
    Market
}


/// # Структура ответа на запрос стакана
/// Используется для получения и хранения стакана.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct OrderBookResponse {
    tracking_id:    str,

    /// Статус.
    status:         str,

    /// Заявка.
    payload:        OrderBook
}


/// Структура заявки.
/// Используется для хранения стакана.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct OrderBook {
    /// Код инструмента.
    figi:                   str,

    /// Глубина.
    depth:                  i32,
    bids:                   [OrderResponse],
    asks:                   [OrderResponse],

    /// Статус торгов.
    trade_status:           TradeStatus,
    min_price_increment:    f32,
    face_value:             f32,
    last_price:             f32,
    close_price:            f32,
    limit_up:               f32,
    limit_down:             f32
}


/// # Структура ответа на заявку
/// Используется для получения и хранения ответа на заювку.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
struct OrderResponse {
    /// Цена.
    price:      f32,
    quantity:   i32
}


/// # Структура ответа на заявки
/// Используется для получения и хранения ответа на заявки
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct OrdersResponse {
    tracking_id:    str,

    /// Статус.
    status:         str,

    /// Массив заявок.
    payload:        [Order]
}


/// # Структура заявки
/// Используется хранения информации о заявке.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct Order {
    order_id:       str,

    /// Код инструмента.
    figi:           str,

    /// Тип операции.
    operation:      OperationType,

    /// Статус.
    status:         OrderStatus,
    requested_lots: u32,
    executed_lots:  u32,

    /// Тип заявки.
    r#type:         OrderType,

    /// Цена.
    price:          f32
}


/// # Структура запроса лимитной заявки
/// Используется для составления лимитной заяви.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
struct LimitOrderRequest {
    /// Количество лотов.
    lots:       u32,

    /// Тип операции.
    operation:  OperationType,

    /// Цена.
    price:      f32
}


/// # Структура ответа лимитной заявки
/// Используется для получения и хранения информации о лимитной заяви.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct LimitOrderResponse {
    tracking_id:    str,

    /// Статус.
    status:         str,
    payload:        PlacedLimitOrder
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct PlacedLimitOrder {
    order_id:       str,
    operation:      OperationType,
    status:         str,
    reject_reason:  str,
    message:        str,
    requested_lots: u32,
    executed_lots:  u32,
    commission:     MoneyAmount
}


#[derive(Serialize, Deserialize, Debug)]
struct MarketOrderRequest {
    lots:       u32,
    operation:  OperationType
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct MarketOrderResponse {
    tracking_id:    str,
    status:         str,
    payload:        PlacedMarketOrder
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct PlacedMarketOrder {
    order_id:       str,
    operation:      OperationType,
    status:         OrderStatus,
    reject_reason:  str,
    message:        str,
    requested_lots: u32,
    executed_lots:  u32,
    commission:     MoneyAmount
}
