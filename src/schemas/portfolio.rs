use crate::schemas::{InstrumentType, MoneyAmount};

use serde::{Deserialize, Serialize};


/// # Структура портфеля
/// Используется для хранения иформации о портфеле.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
#[derive(Serialize, Deserialize, Debug)]
pub struct Portfolio {
    /// Массив позиций в порфеле.
    pub positions: Vec<PortfolioPosition>,
}


/// # Структура позиции в портфеле
/// Используется для хранения информации о позиции.
/// * Может сериализоваться в JSON строку, десериализоваться из JSON строки в перечисление
/// и отлаживаться
/// * Поля преобразуются в стиль `camelCase`, т.к. сервер отправляет названия в таком стиле
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct PortfolioPosition {
    /// Код инструмента.
    pub figi:                           String,
    pub ticker:                         String,

    /// Тип инструмента.
    pub instrument_type:                InstrumentType,

    /// Баланс.
    pub balance:                        f32,
    pub expected_yield:                 MoneyAmount,

    /// Количество лотов.
    pub lots:                           u32,
    pub average_position_price:         MoneyAmount,

    /// Имя.
    pub name:                           String,
}
