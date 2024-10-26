use cosmwasm_std::{Uint128};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Mint { token_id: String, owner: String, amount: Uint128 },
    Transfer { to: String, token_id: String, amount: Uint128 },
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    BalanceOf { owner: String, token_id: String },
}
