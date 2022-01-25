pub mod accounts;
pub mod market;
pub mod operations;
pub mod orders;
pub mod portfolio;
pub mod sandbox;

use std::collections::HashMap;

use chrono::{DateTime, Datelike, Timelike, Utc};
use hyper;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;


pub const STREAMING_URI: &str = "wss://api-invest.tinkoff.ru/openapi/md/v1/md-openapi/ws";

pub const SANDBOX_REGISTER: &str = "https://api-invest.tinkoff.ru/openapi/sandbox/register";
pub const SANDBOX_CURRENCIES_BALANCE: &str = "https://api-invest.tinkoff.ru/openapi/sandbox/currencies/balance";
pub const SANDBOX_POSITIONS_BALANCE: &str = "https://api-invest.tinkoff.ru/openapi/sandbox/positions/balance";
pub const SANDBOX_REMOVE: &str = "https://api-invest.tinkoff.ru/openapi/sandbox/remove";
pub const SANDBOX_CLEAR: &str = "https://api-invest.tinkoff.ru/openapi/sandbox/clear";

pub const ORDERS: &str = "https://api-invest.tinkoff.ru/openapi/orders";
pub const LIMIT_ORDER: &str = "https://api-invest.tinkoff.ru/openapi/orders/limit-order";
pub const MARKET_ORDER: &str = "https://api-invest.tinkoff.ru/openapi/orders/market-order";
pub const CANCEL: &str = "https://api-invest.tinkoff.ru/openapi/orders/cancel";

pub const PORTFOLIO: &str = "https://api-invest.tinkoff.ru/openapi/portfolio";
pub const PORTFOLIO_CURRENCIES: &str = "https://api-invest.tinkoff.ru/openapi/portfolio/currencies";

pub const STOCKS: &str = "https://api-invest.tinkoff.ru/openapi/market/stocks";
pub const BONDS: &str = "https://api-invest.tinkoff.ru/openapi/market/bonds";
pub const ETFS: &str = "https://api-invest.tinkoff.ru/openapi/market/etfs";
pub const CURRENCIES: &str = "https://api-invest.tinkoff.ru/openapi/market/currencies";
pub const ORDER_BOOK: &str = "https://api-invest.tinkoff.ru/openapi/market/orderbook";
pub const CANDLES: &str = "https://api-invest.tinkoff.ru/openapi/market/candles";
pub const BY_FIGI: &str = "https://api-invest.tinkoff.ru/openapi/market/search/by-figi";
pub const BY_TICKER: &str = "https://api-invest.tinkoff.ru/openapi/market/search/by-ticker";

pub const OPERATIONS: &str = "https://api-invest.tinkoff.ru/openapi/operations";
pub const ACCOUNTS: &str = "https://api-invest.tinkoff.ru/openapi/user/accounts";


pub struct Client {
    pub token: String,
    pub hyper_client: hyper::Client<HttpsConnector<HttpConnector>>
}


fn params_str(params: Option<HashMap<String, String>>) -> String {
    return match params {
        Some(some_params) => {
            let mut params_str = String::from("?");
            let mut have_params = false;

            for (k, v) in some_params {
                let mut param = String::from("");

                if have_params {
                    param += "&";
                } 
                else {
                    have_params = true;
                }

                param += format!("{}={}", k, v).as_str();
                params_str.push_str(&param);
            }

            params_str
        },
        None => String::from("")
    }
}

#[cfg(test)]
mod test_params_str {
    use super::params_str;

    use std::collections::HashMap;


    #[test]
    fn default() {
        assert_eq!(params_str(None), "");
    }

    #[test]
    fn broker_account_id() {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("brokerAccountId"), String::from("test"));

        assert_eq!(params_str(Some(params)), "?brokerAccountId=test")
    }
}

pub fn dt_fmt(dt: DateTime<Utc>) -> String {
    format!("{}-{:02}-{:02}T{:02}%3A{:02}%3A{:02}%2B03%3A00",
            dt.year(), dt.month(), dt.day(), dt.hour(), dt.minute(), dt.second())
}

#[cfg(test)]
mod dt_fmt_test {
    use super::dt_fmt;

    #[test]
    fn main() {
        use chrono::prelude::*;

        let dt = Utc.ymd(2019, 8, 19).and_hms(18, 38, 33);
        println!("{}", dt_fmt(dt));

        assert_eq!(dt_fmt(dt), "2019-08-19T18%3A38%3A33%2B03%3A00")
    }
}