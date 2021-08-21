use serde::{Serialize, Deserialize};


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
