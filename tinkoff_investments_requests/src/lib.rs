pub mod accounts;
pub mod market;
pub mod operations;
pub mod orders;
pub mod portfolio;
pub mod sandbox;

use std::collections::HashMap;

use chrono::{DateTime, Datelike, Timelike, Utc};
use hyper::{self, Request, http::request::Builder};
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;


pub const BASE_URI: &str = "https://api-invest.tinkoff.ru/openapi/";
pub const SANDBOX_URI: &str = "https://api-invest.tinkoff.ru/openapi/sandbox/";


pub struct Client {
    pub uri: String,
    pub req: Builder,
    pub hyper_client: hyper::Client<HttpsConnector<HttpConnector>>
}

impl Client {
    pub fn new(token: String, uri: &str) -> Client {
        let https = HttpsConnector::new();
        let hyper_client = hyper::Client::builder()
            .build::<_, hyper::Body>(https);

        let req = Request::builder()
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", &(String::from("Bearer ") + &token));

        Client {
            uri: String::from(uri),
            req,
            hyper_client
        }
    }
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