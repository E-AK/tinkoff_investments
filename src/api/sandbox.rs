use crate::api::{API, SANDBOX_REGISTER, SANDBOX_URI, SANDBOX_CURRENCIES_BALANCE, SANDBOX_CLEAR};
use hyper::{Method, Body};
use crate::schema::sandbox::{SandboxRegisterRequest, SandboxRegisterResponse, SandboxSetCurrencyBalanceRequest, SandboxSetPositionBalanceRequest};
use crate::schema::account::BrokerAccountType;
use crate::schema::error::Error;
use crate::schema::Empty;
use crate::schema::currency::Currency;

/// # Регистрация клинта в Sandbox
/// * `&api: API` - Структура API
/// * `broker_account_type: BrokerAccountType` - Тип счета: `Tinkoff` или `TinkoffIis`
fn register(&api: API, broker_account_type: BrokerAccountType) -> Result<SandboxRegisterResponse, Error> {
    /// Структура запроса
    let req = SandboxRegisterRequest{
        broker_account_type,
    };

    /// Запрос на регистрацию клиента.
    let body = api.req(SANDBOX_URI,
                       SANDBOX_REGISTER,
                       Method::POST,
                       Body::from(req.to_string()))?;

    /// Полученное тело ответа конвертируется в структуру `SandboxRegisterResponse`, в случае ошибки
    /// передаётся ошибка.
    Ok(serde_json::from_slice::<SandboxRegisterResponse>(body.as_ref())?)
}

/// # Выставление баланса по валютным позициям
/// * `&api: API` - Структура API
/// * `currency: Currency` - Валюта
/// * `balance: f32` - Баланс
fn set_currencies_balance(&api: API, currency: Currency, balance: f32) -> Result<Empty<{}>, Error> {
    let req = SandboxSetCurrencyBalanceRequest{
        currency,
        balance
    };

    let body = api.req(SANDBOX_URI,
                       SANDBOX_CURRENCIES_BALANCE,
                       Method::POST,
                       Body::from(req.to_string()))?;

    Ok(serde_json::from_slice::<Empty<{}>>(body.as_ref())?)
}

/// # Выставление баланса по инструментным позициям
/// * `&api: API` - Структура API
/// * `figi: String` - Код бумаги
/// * `balance: f32` - Баланс
fn set_positions_balance(&api: API, figi: String, balance: f32) -> Result<Empty<{}>, Error> {
    let req = SandboxSetPositionBalanceRequest {
        figi,
        balance
    };

    let body = api.req(SANDBOX_URI,
                       SANDBOX_CURRENCIES_BALANCE,
                       Method::POST,
                       Body::from(req.to_string()))?;

    Ok(serde_json::from_slice::<Empty<{}>>(body.as_ref())?)
}

/// # Удаление счета
/// * `&api: API` - Структура API
fn remove(&api: API) -> Result<Empty<{}>, Error> {
    let body = api.req(SANDBOX_URI,
                       SANDBOX_REGISTER,
                       Method::POST,
                       Body::empty())?;

    Ok(serde_json::from_slice::<Empty<{}>>(body.as_ref())?)
}

/// # Удаление всех позиций
/// * `&api: API` - Структура API
fn clear(&api: API) -> Result<Empty<{}>, Error> {
    let body = api.req(SANDBOX_URI,
                       SANDBOX_CLEAR,
                       Method::POST,
                       Body::empty())?;

    Ok(serde_json::from_slice::<Empty<{}>>(body.as_ref())?)
}
