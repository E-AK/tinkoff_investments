use serde::{Serialize, Deserialize};
use crate::schema::candle::{CandleResolution, Candle};

#[derive(Serialize, Deserialize, Debug)]
enum Type {
    Candle,
    OrderBook,
    InstrumentInfo,
}

#[derive(Serialize, Deserialize, Debug)]
enum Event {
    Subscribe(Type),
    Unsubscribe(Type),
}

#[derive(Serialize, Deserialize, Debug)]
struct CandleReq {
    event: Event,
    figi: str,
    interval: CandleResolution,
}

#[derive(Serialize, Deserialize, Debug)]
struct CandleResp {
    event: str,
    time: str,
    payload: Candle,
}

#[derive(Serialize, Deserialize, Debug)]
struct Order {
    depth: u8,
    bids: [f32],
    asks: [f32],
    figi: str,
}

#[derive(Serialize, Deserialize, Debug)]
struct OrderBookReq {
    event: str,
    time: str,
    payload: Order,
}

#[derive(Serialize, Deserialize, Debug)]
struct OrderBookResp {
    event: str,
    figi: str,
    depth: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct InstrumentInfoReq {
    event: str,
    figi: str,
}

#[derive(Serialize, Deserialize, Debug)]
struct InstrumentType {
    trade_status: str,
    min_price_increment: str,
    lot: u32,
    accrued_interest: f32,
    limit_up: f32,
    limit_down: f32,
    figi: str,
}

#[derive(Serialize, Deserialize, Debug)]
struct InstrumentInfoResp {
    event: str,
    time: str,
    payload: InstrumentType,
}

#[derive(Serialize, Deserialize, Debug)]
struct Error {
    error: str,
    request_id: str,
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorResp {
    event: str,
    time: str,
    payload: Error,
}