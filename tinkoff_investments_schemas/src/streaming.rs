use serde::{Serialize, Deserialize};


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
    pub interval: super::candle::CandleResolution,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CandleResp {
    pub event: String,
    pub time: String,
    pub payload: super::candle::Candle,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct OrderBookReq {
    pub event: String,
    pub time: String,
    pub payload: super::order::Order,
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