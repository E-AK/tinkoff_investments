pub mod account;
pub mod order;
pub mod market;
pub mod operation;
pub mod currency;
pub mod error;
pub mod candle;
pub mod sandbox;
pub mod streaming;
pub mod portfolio;


use serde::{Serialize, Deserialize};


enum RequestTypes {
    LimitOrderRequest(order::LimitOrderRequest),
    MarketOrderRequest(order::MarketOrderRequest),
    SandboxRegisterRequest(sandbox::SandboxRegisterRequest),
    SandboxSetCurrencyBalanceRequest(sandbox::SandboxSetCurrencyBalanceRequest),
    SandboxSetPositionBalanceRequest(sandbox::SandboxSetPositionBalanceRequest),
}

enum ResponseTypes {
    Empty(Response),
    Error(Response),
    SandboxRegisterResponse(Response),
    OrdersResponse(Response),
    LimitOrderResponse(Response),
    MarketOrderResponse(Response),
    PortfolioResponse(Response),
    PortfolioCurrenciesResponse(Response),
    UserAccountsResponse(Response),
    OrderbookResponse(Response),
    OrderResponse(Response),
    CandlesResponse(Response),
    OperationsResponse(Response),
    MarketInstrumentListResponse(Response),
    SearchMarketInstrumentResponse(Response),
    MarketInstrumentResponse(Response)
}

enum PayloadTypes {
    Empty,

}

/// # Структура ответа на запрос
/// Используется для получения и хранения информации о брокерских счетах.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в стуктуру
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Response {
    pub tracking_id:    String,
    pub status:         String,
    pub payload:        PayloadTypes
}


/// # Тип брокерского счета
/// Используется для выбора брокерского счета или ИИС.
#[derive(Serialize, Deserialize, Debug)]
pub enum BrokerAccountType {
    /// Брокерский счет.
    Tinkoff,

    /// ИИС.
    TinkoffIis
}

/// # Структура с массивом брокерских счетов
/// Используется, как тип данных при получении данных.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserAccounts {
    /// Список брокерских счетов.
    pub accounts: Vec<UserAccount>
}


/// # Структура брокерского счета
/// Структура хранит информацию о брокерском счете.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct UserAccount {
    /// Тип брокерского счета.
    pub broker_account_type:    BrokerAccountType,

    /// Идентификатор (id) брокерского счета.
    pub broker_account_id:      String
}


/// # Интервал свечей
/// Используется для выбора интервала свечей.
#[derive(Serialize, Deserialize, Debug)]
pub enum CandleResolution {
    /// `1min`
    #[serde(rename="1min")]
    Min,

    /// `2min`
    #[serde(rename="2min")]
    TwoMin,

    /// `3min`
    #[serde(rename="3min")]
    ThreeMin,

    /// `5min`
    #[serde(rename="5min")]
    FiveMin,

    /// `10min`
    #[serde(rename="10min")]
    TenMin,

    /// `15min`
    #[serde(rename="15min")]
    FifteenMin,

    /// `30min`
    #[serde(rename="30min")]
    ThirtyMin,

    /// `hour`
    #[serde(rename="hour")]
    Hour,

    /// `day`
    #[serde(rename="day")]
    Day,

    /// `week`
    #[serde(rename="week")]
    Week,

    /// `month`
    #[serde(rename="month")]
    Month
}


/// # Структура свечей
/// Используется для хранения информации о свечах.
#[derive(Serialize, Deserialize, Debug)]
pub struct Candles {
    /// Код инструмента.
    pub figi:       String,

    /// Интервал.
    pub interval:   CandleResolution,

    /// Массив свечей.
    pub candles:    Vec<Candle>
}


/// # Структура свечи
/// Испольуется для хранения информации о свечи.
#[derive(Serialize, Deserialize, Debug)]
pub struct Candle {
    /// Код инструмента.
    pub figi:       String,

    /// Интервал.
    pub interval:   CandleResolution,

    /// Цена открытия.
    pub o:          f32,

    /// Цена закрытия.
    pub c:          f32,

    /// Наивысшая цена.
    pub h:          f32,

    /// Наименьшая цена.
    pub l:          f32,

    /// Объем.
    pub v:          f32,

    /// Дата и время.
    pub time:      String
}


/// # Валюта
/// Используется для выбора валюты.
#[derive(Serialize, Deserialize, Debug)]
pub enum Currency {
    RUB,
    USD,
    EUR,
    GBP,
    HKG,
    CHF,
    JPY,
    CNY,
    TRY
}


/// # Структура списка валют
/// Используется для хранения массива валютных позиций.
#[derive(Serialize, Deserialize, Debug)]
pub struct Currencies {
    /// Массив валютных позиций.
    pub currencies: Vec<CurrencyPosition>
}


/// # Структура позиции по валюте
/// Используется для хранения валютных позиций.
#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyPosition {
    /// Валюта.
    pub currency:   Currency,

    /// Баланс.
    pub balance:    f32,

    #[serde(default)]
    pub blocked:   f32
}


/// # Структура информации об ошибке
/// Используется для хранения информации об ошибке.
#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    /// Сообщение.
    pub message:    String,

    /// Статус код.
    pub code:       String
}


/// # Структура списка инструментов
/// Используется для храниния массива инструментов.
#[derive(Serialize, Deserialize, Debug)]
pub struct MarketInstrumentList {
    pub total:          i32,

    /// Массив инструментов.
    pub instruments:    Vec<MarketInstrument>
}


/// # Информация об инструменте
/// Используется для хранения информации об инструменте.
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


/// # Тип операции
/// Используется для выбора операции.
#[derive(Serialize, Deserialize, Debug)]
pub enum OperationType {
    /// Покупка.
    Buy,

    /// Продажа.
    Sell
}

/// # Тип операций, включая комиссию
/// Используется для указания операции.
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
#[derive(Serialize, Deserialize, Debug)]
pub struct Operations {
    /// Операции.
    pub operations: Vec<Operation>
}

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


/// # Структура портфеля
/// Используется для хранения иформации о портфеле.
#[derive(Serialize, Deserialize, Debug)]
pub struct Portfolio {
    /// Массив позиций в порфеле.
    pub positions: Vec<PortfolioPosition>,
}


/// # Структура позиции в портфеле
/// Используется для хранения информации о позиции.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct PortfolioPosition {
    /// Код инструмента.
    pub figi:                           String,
    pub ticker:                         String,

    /// Тип инструмента.
    pub instrument_type:                InstrumentType,

    /// Баланс.
    pub balance:                        f32,
    pub expected_yield:                 MoneyAmount,

    /// Количество лотов.
    pub lots:                           u32,
    pub average_position_price:         MoneyAmount,

    /// Имя.
    pub name:                           String,
}


/// # Структура запроса регистрации песочницы
/// Используется для созтавления запроса регистрации песочницы.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct SandboxRegisterRequest {
    pub broker_account_type: BrokerAccountType
}


/// # Структура запроса установки баланса в песочнице
/// Используется для составления запроса установки баланса в песочнице.
#[derive(Serialize, Deserialize, Debug)]
pub struct SandboxSetCurrencyBalanceRequest {
    /// Валюта.
    pub currency:   Currency,

    /// Баланс.
    pub balance:    f32
}


/// # Структура запроса установки позиции в песочнице
/// Используется для составления запроса установки позиции в песочнице.
#[derive(Serialize, Deserialize, Debug)]
pub struct SandboxSetPositionBalanceRequest {
    /// Код инструмента.
    pub figi:       String,

    /// Баланс.
    pub balance:    f32
}


/// # Структура информации о средствах.
/// Используется для указания средств.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в стуктуру
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
pub struct MoneyAmount {
    /// Валюта.
    pub currency: Currency,

    /// Значение.
    pub value: f32,
}


/// # Статус торгов
/// Используется для указания статуса торгов.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в стуктуру
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
pub enum TradeStatus {
    /// Нормальная торговля.
    NormalTrading,

    /// Недоступно для торговли.
    NotAvailableForTrading,
}


/// # Тип инструмента
/// Используется для выбора типа бумаги.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в стуктуру
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
pub enum InstrumentType {
    /// Акции.
    Stock,

    /// Валюта.
    Currency,

    /// Облигации.
    Bond,

    /// Фонды.
    Etf,
}


#[derive(Serialize, Deserialize, Debug)]
pub enum Type {
    Candle,
    OrderBook,
    InstrumentInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    Subscribe(Type),
    Unsubscribe(Type),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CandleReq {
    pub event: Event,
    pub figi: String,
    pub interval: CandleResolution,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CandleResp {
    pub event: String,
    pub time: String,
    pub payload: Candle,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub depth: u8,
    pub bids: Vec<f32>,
    pub asks: Vec<f32>,
    pub figi: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderBookReq {
    pub event: String,
    pub time: String,
    pub payload: Order,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderBookResp {
    pub event: String,
    pub figi: String,
    pub depth: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstrumentInfoReq {
    pub event: String,
    pub figi: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstrumentType {
    pub trade_status: String,
    pub min_price_increment: String,
    pub lot: u32,
    pub accrued_interest: f32,
    pub limit_up: f32,
    pub limit_down: f32,
    pub figi: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstrumentInfoResp {
    pub event: String,
    pub time: String,
    pub payload: InstrumentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub error: String,
    pub request_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResp {
    pub event: String,
    pub time: String,
    pub payload: Error,
}