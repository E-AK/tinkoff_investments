use serde::{Serialize, Deserialize};


/// # Структура информации об ошибке
/// Используется для хранения информации об ошибке.
#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    /// Сообщение.
    pub message:    String,

    /// Статус код.
    pub code:       String
}
