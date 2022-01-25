use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct MarketInstrumentList {
    pub total:                  i32,

    /// Массив инструментов.
    pub instruments:            Vec<MarketInstrument>
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct OrderBook {
    /// Код инструмента.
    pub figi:                   String,

    /// Глубина.
    pub depth:                  i32,
    pub bids:                   Vec<super::ResponseTypes>,
    pub asks:                   Vec<super::ResponseTypes>,

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


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct SearchMarketInstrument {
    pub figi:                   String,
    pub ticker:                 String,
    pub isin:                   String,
    pub min_price_increment:      f32,
    pub lot:                    i32,
    pub currency:               super::currency::Currency,
    pub name:                   String,
    pub r#type:                 super::InstrumentType,
}

pub struct OrderResponse {
    pub price:                  f32,
    pub quantity:               i32,
}

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
    pub currency:               super::currency::Currency,

    /// Название бумаги.
    pub name:                   String,

    /// Тип инструмента.
    pub r#type:                 super::InstrumentType
}


#[derive(Serialize, Deserialize, Debug)]
pub enum TradeStatus {
    /// Нормальная торговля.
    NormalTrading,

    /// Недоступно для торговли.
    NotAvailableForTrading,
}