#[derive(Serialize, Deserialize, Debug)]
pub struct LimitOrderRequest {
    /// Количество лотов.
    pub lots:                   u32,

    /// Тип операции.
    pub operation:              super::OperationType,

    /// Цена.
    pub price:                  f32
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub depth:                  u8,
    pub bids:                   Vec<f32>,
    pub asks:                   Vec<f32>,
    pub figi:                   String,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct PlacedMarketOrder {
    pub order_id:               String,
    pub operation:              OperationType,
    pub status:                 OrderStatus,

    #[serde(default)]
    pub reject_reason:          String,

    #[serde(default)]
    pub message:                String,
    pub requested_lots:         u32,
    pub executed_lots:          u32,
    pub commission:             super::MoneyAmount
}


#[derive(Serialize, Deserialize, Debug)]
pub struct MarketOrderRequest {
    pub lots:                   u32,
    pub operation:              OperationType
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct PlacedLimitOrder {
    pub order_id:               String,
    pub operation:              OperationType,
    pub status:                 String,
    pub reject_reason:          String,
    pub message:                String,
    pub requested_lots:         u32,
    pub executed_lots:          u32,
    pub commission:             super::MoneyAmount
}


#[derive(Serialize, Deserialize, Debug)]
pub enum OperationType {
    /// Покупка.
    Buy,

    /// Продажа.
    Sell
}


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
    pub r#type:         super::OrderType,

    /// Цена.
    pub price:          f32
}


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


#[derive(Serialize, Deserialize, Debug)]
pub enum OrderType {
    /// Лимитная.
    Limit,

    /// Рыночная.
    Market
}