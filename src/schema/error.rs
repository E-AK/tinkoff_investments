use serde::{Serialize, Deserialize};


/// # Структура информации об ошибке
/// Используется для хранения информации об ошибке.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
struct ErrorPayload {
    /// Сообщение.
    message:    str,

    /// Статус код.
    code:       str
}


/// # Структура ошибки, полученной при запросе
/// Используется для получения ошибки.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Error {
    tracking_id:    str,

    /// Статус.
    status:         str,

    /// Информация об ошибке.
    payload:        ErrorPayload
}
