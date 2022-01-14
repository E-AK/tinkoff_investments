#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct SandboxRegisterRequest {
    pub broker_account_type:    super::BrokerAccountType
}


#[derive(Serialize, Deserialize, Debug)]
pub struct SandboxSetCurrencyBalanceRequest {
    /// Валюта.
    pub currency:               super::Currency,

    /// Баланс.
    pub balance:                f32
}


#[derive(Serialize, Deserialize, Debug)]
pub struct SandboxSetPositionBalanceRequest {
    /// Код инструмента.
    pub figi:                   String,

    /// Баланс.
    pub balance:                f32
}

pub struct SandboxAccount {
    brokerAccountType:          super::BrokerAccountType,
    brokerAccountId:            String
}