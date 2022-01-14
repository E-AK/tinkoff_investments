pub mod account;
pub mod order;
pub mod market;
pub mod operation;
pub mod currency;
pub mod candle;
pub mod sandbox;
pub mod streaming;
pub mod portfolio;


pub enum RequestTypes {
    LimitOrderRequest(self::order::LimitOrderRequest),
    MarketOrderRequest(self::market::MarketOrderRequest),
    SandboxRegisterRequest(self::sandbox::SandboxRegisterRequest),
    SandboxSetCurrencyBalanceRequest(self::sandbox::SandboxSetCurrencyBalanceRequest),
    SandboxSetPositionBalanceRequest(self::sandbox::SandboxSetPositionBalanceRequest),
}

pub enum ResponseTypes {
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
    MarketInstrumentResponse(Response),
}

enum PayloadTypes {
    Empty,
    Error(ErrorPayload),
    SandboxAccount(self::sandbox::SandboxAccount),
    Order(Vec<self::order::Order>),
    PlacedLimitOrder(self::order::PlacedLimitOrder),
    PlacedMarketOrder(self::order::PlacedMarketOrder),
    Portfolio(sefl::portfolio::Portfolio),
    Currencies(self::currency::Currencies),
    MarketInstrumentList(self::market::MarketInstrumentList),
    OrderBook(self::market::OrderBook),
    Candles(self::candle::Candles),
    SearchMarketInstrument(self::market::SearchMarketInstrument),
    Operations(self::operation::Operations),
    Useraccounts(self::account::UserAccounts),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Response {
    pub tracking_id:            String,
    pub status:                 String,
    pub payload:                PayloadTypes
}

struct ErrorPayload {
    message:                    String,
    code:                       String,
}


#[derive(Serialize, Deserialize, Debug)]
pub enum BrokerAccountType {
    /// Брокерский счет.
    Tinkoff,

    /// ИИС.
    TinkoffIis
}


/// # Структура информации о средствах.
/// Используется для указания средств.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в стуктуру
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
pub struct MoneyAmount {
    /// Валюта.
    pub currency:               self::currency::Currency,

    /// Значение.
    pub value:                  f32,
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