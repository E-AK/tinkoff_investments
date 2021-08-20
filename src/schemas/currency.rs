use serde::{Serialize, Deserialize};


/// # Валюта
/// Используется для выбора валюты.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
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
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
pub struct Currencies {
    /// Массив валютных позиций.
    pub currencies: Vec<CurrencyPosition>
}


/// # Структура позиции по валюте
/// Используется для хранения валютных позиций.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyPosition {
    /// Валюта.
    pub currency:   Currency,

    /// Баланс.
    pub balance:    f32,

    #[serde(default)]
    pub blocked:   f32
}
