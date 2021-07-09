use serde::{Serialize, Deserialize};
use crate::schema::{MoneyAmount, Currency, InstrumentType};

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
enum OperationTypeWithCommission {
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
enum OperationStatus {
    /// Выполнено.
    Done,

    /// Отклонено.
    Decline,

    /// Ожидание.
    Progress
}

/// # Структура ответа на запрос об операциях
/// Используется для получения и хранения информации об операциях
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct OperationsResponse {
    tracking_id:    str,

    /// Статус.
    status:         str,

    /// Операции.
    payload:        Operations
}

/// # Структура списка операций
/// Используется для хранения массива операций.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
struct Operations {
    /// Операции.
    operations: [Operation]
}

/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct OperationTrade {
    trade_id:   str,

    /// Дата и время.
    date:       str,

    /// Цена.
    price:      f32,
    quantity:   i32
}

/// # Структура операции
/// Используется для хранения информации об операции.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct Operation {
    id:         str,

    /// Статус.
    status:             OperationStatus,
    trades:             [OperationTrade],

    /// Комиссия.
    commission:         MoneyAmount,

    /// Валюта.
    currency:           Currency,
    payment:            f32,

    /// Цена.
    price:              f32,
    quantity:           i32,
    quantity_executed:  i32,

    /// Код инструмента.
    figi: str,

    /// Тип инструмента.
    instrument_type:    InstrumentType,
    is_margin_call:     bool,

    /// Дата и время.
    date: str,

    /// Тип операции.
    operation_type: OperationTypeWithCommission
}
