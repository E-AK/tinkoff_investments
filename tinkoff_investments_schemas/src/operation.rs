use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Operations {
    /// Операции.
    pub operations:             Vec<Operation>
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Operation {
    pub id:                     String,

    /// Статус.
    pub status:                 OperationStatus,
    pub trades:                 Vec<OperationTrade>,

    /// Комиссия.
    pub commission:             super::MoneyAmount,

    /// Валюта.
    pub currency:               super::currency::Currency,
    pub payment:                f32,

    /// Цена.
    pub price:                  f32,
    pub quantity:               i32,
    pub quantity_executed:      i32,

    /// Код инструмента.
    pub figi:                   String,

    /// Тип инструмента.
    pub instrument_type:        super::InstrumentType,
    pub is_margin_call:         bool,

    /// Дата и время.
    pub date:                   String,

    /// Тип операции.
    pub operation_type:         OperationTypeWithCommission
}


#[derive(Serialize, Deserialize, Debug)]
pub enum OperationStatus {
    /// Выполнено.
    Done,

    /// Отклонено.
    Decline,

    /// Ожидание.
    Progress
}


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


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct OperationTrade {
    pub trade_id:               String,

    /// Дата и время.
    pub date:                   String,

    /// Цена.
    pub price:                  f32,
    pub quantity:               i32
}