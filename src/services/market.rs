use crate::Service;
use crate::services::{BASE_URI, STOCKS, BONDS, ETFS, CURRENCIES, ORDER_BOOK, CANDLES, BY_FIGI, BY_TICKER, dt_fmt};
use crate::schemas::candle::{CandleResolution, Candles};

use hyper::{Method, Body};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::schemas::error::Error;
use crate::schemas::Resp;
use crate::schemas::market::{MarketInstrumentList, MarketInstrument, OrderBook};


impl Service {
    /// # Получение списка акций
    /// * `api` - Объект API
    pub async fn get_stocks(&self) -> Result<Resp<MarketInstrumentList>, Resp<Error>> {
        let data = self.req(
            BASE_URI.to_string(),
            STOCKS.to_string(),
            Method::GET,
            HashMap::new(),
            Body::empty()).await;

        return match data {
            Ok(obj) =>
                Ok(serde_json::from_str::<Resp<MarketInstrumentList>>(&obj).unwrap()),
            Err(err) => Err(err)
        }
    }

    /// # Получение списка облигаций
    pub async fn get_bonds(&self) -> Result<Resp<MarketInstrumentList>, Resp<Error>> {
        let data = self.req(
            BASE_URI.to_string(),
            BONDS.to_string(),
            Method::GET,
            HashMap::new(),
            Body::empty()).await;

        return match data {
            Ok(obj) =>
                Ok(serde_json::from_str::<Resp<MarketInstrumentList>>(&obj).unwrap()),
            Err(err) => Err(err)
        }
    }

    /// # Получение списка ETF
    pub async fn get_etfs(&self) -> Result<Resp<MarketInstrumentList>, Resp<Error>> {
        let data = self.req(
            BASE_URI.to_string(),
            ETFS.to_string(),
            Method::GET,
            HashMap::new(),
            Body::empty()).await;

        return match data {
            Ok(obj) =>
                Ok(serde_json::from_str::<Resp<MarketInstrumentList>>(&obj).unwrap()),
            Err(err) => Err(err)
        }
    }

    /// # Получение списка валютных пар
    pub async fn get_currencies(&self) -> Result<Resp<MarketInstrumentList>, Resp<Error>> {
        let data = self.req(
            BASE_URI.to_string(),
            CURRENCIES.to_string(),
            Method::GET,
            HashMap::new(),
            Body::empty()).await;

        return match data {
            Ok(obj) =>
                Ok(serde_json::from_str::<Resp<MarketInstrumentList>>(&obj).unwrap()),
            Err(err) => Err(err)
        }
    }

    /// # Получение стакана по FIGI
    /// * `figi` - Код инструмента для фильтрации
    /// * `depth` - Глубина стакана [1..20]
    pub async fn get_order_book(&self, figi: String, depth: u8) -> Result<Resp<OrderBook>, Resp<Error>> {
        let mut params = HashMap::new();

        params.insert("figi".to_string(), figi);
        params.insert("depth".to_string(), depth.to_string());

        let data = self.req(
            BASE_URI.to_string(),
            ORDER_BOOK.to_string(),
            Method::GET,
            params,
            Body::empty()).await;

        return match data {
            Ok(obj) =>
                Ok(serde_json::from_str::<Resp<OrderBook>>(&obj).unwrap()),
            Err(err) => Err(err)
        }
    }

    /// # Получение исторических свечей по FIGI
    /// * `figi` - Код инструмента для фильтрации
    /// * `from` - Начало временного промежутка
    /// * `to` - Конец временного промежутка
    /// * `interval` - Интервал свечи
    ///     * 1min  -    Максимум 1 день
    ///     * 2min  -    Максимум 1 день
    ///     * 3min  -    Максимум 1 день
    ///     * 5min  -    Максимум 1 день
    ///     * 10min -    Максимум 1 день
    ///     * 15min -    Максимум 1 день
    ///     * 30min -    Максимум 1 день
    ///     * hour  -    Максимум 7 дней
    ///     * day   -    Максимум 1 год
    ///     * week  -    Максимум 2 года
    ///     * month -    Максимум 10 лет
    pub async fn get_candles(
        &self, figi: String, from: DateTime<Utc>, to: DateTime<Utc>, interval: CandleResolution
    ) -> Result<Resp<Candles>, Resp<Error>>
    {
        let mut params = HashMap::new();

        let inter = serde_json::to_string(&interval).unwrap();
        let _inter = &inter[1..inter.len()-1];

        params.insert("figi".to_string(), figi);
        params.insert("from".to_string(), dt_fmt(from));
        params.insert("to".to_string(), dt_fmt(to));
        params.insert("interval".to_string(), _inter.to_string());

        let data = self.req(
            BASE_URI.to_string(),
            CANDLES.to_string(),
            Method::GET,
            params,
            Body::empty()).await;

        return match data {
            Ok(obj) =>
                Ok(serde_json::from_str::<Resp<Candles>>(&obj).unwrap()),
            Err(err) => Err(err)
        }
    }

    /// # Получение инструмента по FIGI
    /// * `figi` - Код инструмента
    pub async fn get_by_figi(&self, figi: String) -> Result<Resp<MarketInstrument>, Resp<Error>> {
        let mut params = HashMap::new();

        params.insert("figi".to_string(), figi);

        let data = self.req(
            BASE_URI.to_string(),
            BY_FIGI.to_string(),
            Method::GET,
            params, Body::empty()).await;

        return match data {
            Ok(obj) =>
                Ok(serde_json::from_str::<Resp<MarketInstrument>>(&obj).unwrap()),
            Err(err) => Err(err)
        }
    }

    /// # Получение инструмента по тикеру
    /// * `ticker` - Тикер
    pub async fn get_by_ticker(&self, ticker: String) -> Result<Resp<MarketInstrumentList>, Resp<Error>> {
        let mut params = HashMap::new();

        params.insert("ticker".to_string(), ticker);

        let data = self.req(
            BASE_URI.to_string(),
            BY_TICKER.to_string(),
            Method::GET,
            params,
            Body::empty()).await;

        return match data {
            Ok(obj) =>
                Ok(serde_json::from_str::<Resp<MarketInstrumentList>>(&obj).unwrap()),
            Err(err) => Err(err)
        }
    }
}
