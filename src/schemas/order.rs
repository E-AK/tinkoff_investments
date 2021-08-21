use crate::schemas::MoneyAmount;
use crate::schemas::operation::OperationType;

use serde::{Serialize, Deserialize};


/// # Статус заявки
/// Используется для отображения статуса заявки.
#[derive(Serialize, Deserialize, Debug)]
pub enum OrderStatus {
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
#[derive(Serialize, Deserialize, Debug)]
pub enum OrderType {
    /// Лимитная.
    Limit,

    /// Рыночная.
    Market
}


/// # Структура заявки
/// Используется хранения информации о заявке.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Order {
    pub order_id:       String,

    /// Код инструмента.
    pub figi:           String,

    /// Тип операции.
    pub operation:      OperationType,

    /// Статус.
    pub status:         OrderStatus,
    pub requested_lots: u32,
    pub executed_lots:  u32,

    /// Тип заявки.
    pub r#type:         OrderType,

    /// Цена.
    pub price:          f32
}


/// # Структура запроса лимитной заявки
/// Используется для составления лимитной заяви.
#[derive(Serialize, Deserialize, Debug)]
pub struct LimitOrderRequest {
    /// Количество лотов.
    pub lots:       u32,

    /// Тип операции.
    pub operation:  OperationType,

    /// Цена.
    pub price:      f32
}


/// # Структура ответа лимитной заявки
/// Используется для получения и хранения лимитной заяви.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct PlacedLimitOrder {
    pub order_id:       String,
    pub operation:      OperationType,
    pub status:         String,
    pub reject_reason:  String,
    pub message:        String,
    pub requested_lots: u32,
    pub executed_lots:  u32,

    pub commission:     MoneyAmount
}


/// # Структура запроса рыночной заявки
/// Используется для составления рыночной заяви.
#[derive(Serialize, Deserialize, Debug)]
pub struct MarketOrderRequest {
    pub lots:       u32,
    pub operation:  OperationType
}


/// # Структура ответа лимитной заявки
/// Используется для получения и хранения рыночной заяви.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct PlacedMarketOrder {
    pub order_id:       String,
    pub operation:      OperationType,
    pub status:         OrderStatus,

    #[serde(default)]
    pub reject_reason:  String,

    #[serde(default)]
    pub message:        String,
    pub requested_lots: u32,
    pub executed_lots:  u32,
    pub commission:     MoneyAmount
}
