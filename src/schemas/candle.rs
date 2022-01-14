#[derive(Serialize, Deserialize, Debug)]
pub enum CandleResolution {
    /// `1min`
    #[serde(rename="1min")]
    Min,

    /// `2min`
    #[serde(rename="2min")]
    TwoMin,

    /// `3min`
    #[serde(rename="3min")]
    ThreeMin,

    /// `5min`
    #[serde(rename="5min")]
    FiveMin,

    /// `10min`
    #[serde(rename="10min")]
    TenMin,

    /// `15min`
    #[serde(rename="15min")]
    FifteenMin,

    /// `30min`
    #[serde(rename="30min")]
    ThirtyMin,

    /// `hour`
    #[serde(rename="hour")]
    Hour,

    /// `day`
    #[serde(rename="day")]
    Day,

    /// `week`
    #[serde(rename="week")]
    Week,

    /// `month`
    #[serde(rename="month")]
    Month
}


/// # Структура свечей
/// Используется для хранения информации о свечах.
#[derive(Serialize, Deserialize, Debug)]
pub struct Candles {
    /// Код инструмента.
    pub figi:       String,

    /// Интервал.
    pub interval:   CandleResolution,

    /// Массив свечей.
    pub candles:    Vec<Candle>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Candle {
    /// Код инструмента.
    pub figi:                   String,

    /// Интервал.
    pub interval:               CandleResolution,

    /// Цена открытия.
    pub o:                      f32,

    /// Цена закрытия.
    pub c:                      f32,

    /// Наивысшая цена.
    pub h:                      f32,

    /// Наименьшая цена.
    pub l:                      f32,

    /// Объем.
    pub v:                      f32,

    /// Дата и время.
    pub time:                   String
}