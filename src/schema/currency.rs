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
    currencies: [CurrencyPosition]
}


/// # Структура позиции по валюте
/// Используется для хранения валютных позиций.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
struct CurrencyPosition {
    /// Валюта.
    currency:   Currency,

    /// Баланс.
    balance:    f32,

    /// Заблокировано средств.
    blocked:    f32
}
