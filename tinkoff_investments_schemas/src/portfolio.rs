use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Portfolio {
    /// Массив позиций в порфеле.
    pub positions:              Vec<PortfolioPosition>,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct PortfolioPosition {
    /// Код инструмента.
    pub figi:                           String,
    pub ticker:                         String,

    /// Тип инструмента.
    pub instrument_type:                super::InstrumentType,

    /// Баланс.
    pub balance:                        f32,
    pub expected_yield:                 super::MoneyAmount,

    /// Количество лотов.
    pub lots:                           u32,
    pub average_position_price:         super::MoneyAmount,

    /// Имя.
    pub name:                           String,
}