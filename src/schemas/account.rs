#[derive(Serialize, Deserialize, Debug)]
pub struct UserAccounts {
    /// Список брокерских счетов.
    pub accounts:               Vec<UserAccount>
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct UserAccount {
    /// Тип брокерского счета.
    pub broker_account_type:    super::BrokerAccountType,

    /// Идентификатор (id) брокерского счета.
    pub broker_account_id:      String
}