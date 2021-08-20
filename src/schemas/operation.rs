use crate::schemas::{MoneyAmount, InstrumentType};
use crate::schemas::currency::Currency;

use serde::{Serialize, Deserialize};


/// # Тип операции
/// Используется для выбора операции.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
pub enum OperationType {
    /// Покупка.
    Buy,

    /// Продажа.
    Sell
}

/// # Тип операций, включая комиссию
/// Используется для указания операции.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
pub enum OperationTypeWithCommission {
    Buy,
    BuyCard,
    Sell,
    BrokerCommission,
    ExchangeCommission,
    ServiceCommission,
    MarginCommission,
    OtherCommission,
    PayIn,
    PayOut,
    Tax,
    TaxLucre,
    TaxDividend,
    TaxCoupon,
    TaxBack,
    Repayment,
    PartRepayment,
    Coupon,
    Dividend,
    SecurityIn,
    SecurityOut
}

/// # Статус операции
/// Используется для указания статуса операции.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
pub enum OperationStatus {
    /// Выполнено.
    Done,

    /// Отклонено.
    Decline,

    /// Ожидание.
    Progress
}


/// # Структура списка операций
/// Используется для хранения массива операций.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
pub struct Operations {
    /// Операции.
    pub operations: Vec<Operation>
}

/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct OperationTrade {
    pub trade_id:   String,

    /// Дата и время.
    pub date:       String,

    /// Цена.
    pub price:      f32,
    pub quantity:   i32
}

/// # Структура операции
/// Используется для хранения информации об операции.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Operation {
    pub id:                 String,

    /// Статус.
    pub status:             OperationStatus,
    pub trades:             Vec<OperationTrade>,

    /// Комиссия.
    pub commission:         MoneyAmount,

    /// Валюта.
    pub currency:           Currency,
    pub payment:            f32,

    /// Цена.
    pub price:              f32,
    pub quantity:           i32,
    pub quantity_executed:  i32,

    /// Код инструмента.
    pub figi:               String,

    /// Тип инструмента.
    pub instrument_type:    InstrumentType,
    pub is_margin_call:     bool,

    /// Дата и время.
    pub date:               String,

    /// Тип операции.
    pub operation_type:     OperationTypeWithCommission
}
