use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

///Messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub trading_incentives_code_id: u64,
    pub offer_code_id: u64,
    pub trade_code_id: u64,
    pub warchest_addr: String,
    pub local_pool_addr: String,
    pub local_token_addr: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub trade_code_id: u64,
    pub local_token_addr: Addr,
    pub local_pool_addr: Addr,
    pub warchest_addr: Addr,
    pub offers_addr: Addr,
    pub trading_incentives_addr: Addr,
}
