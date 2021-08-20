use serde::{Serialize, Deserialize};


/// # Тип брокерского счета
/// Используется для выбора брокерского счета или ИИС.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
pub enum BrokerAccountType {
    /// Брокерский счет.
    Tinkoff,

    /// ИИС.
    TinkoffIis
}

/// # Структура с массивом брокерских счетов
/// Используется, как тип данных при получении данных.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в структуру
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
pub struct UserAccounts {
    /// Список брокерских счетов.
    pub accounts: Vec<UserAccount>
}


/// # Структура брокерского счета
/// Структура хранит информацию о брокерском счете.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в структуру
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase` т.к. сервер отправляет названия в таком стиле
/// * Есть алиас с именем `SandboxAccount`, т.к. есть структура `SandboxAccount` с такими же полями
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
// #[serde(alias="SandboxAccount")]
pub struct UserAccount {
    /// Тип брокерского счета.
    pub broker_account_type:    BrokerAccountType,

    /// Идентификатор (id) брокерского счета.
    pub broker_account_id:      String
}
