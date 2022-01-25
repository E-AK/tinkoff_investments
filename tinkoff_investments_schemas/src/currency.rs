use serde::{Serialize, Deserialize};


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


#[derive(Serialize, Deserialize, Debug)]
pub struct Currencies {
    /// Массив валютных позиций.
    pub currencies: Vec<CurrencyPosition>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyPosition {
    /// Валюта.
    pub currency:   Currency,

    /// Баланс.
    pub balance:    f32,

    #[serde(default)]
    pub blocked:   f32
}