use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub native_token: String,
    pub fee_address: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        native_token: String,
        fee_address: Addr,
        fees_percentage: u64,
    },
    Receive(Cw20ReceiveMsg),
    Unstake {
        denom: String,
    },
}

#[cw_serde]
pub enum LiquidityReceiveMsg {
    Lock {
        owner: Addr,
        denom: String,
        locktime: u64,
        amount: Uint128,
    },
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub enabled: bool,
    pub fees_percentage: u64,
}

#[cw_serde]
pub struct LiquidityResponse {
    pub owner: Addr,
    pub denom: String,
    pub locktime: Uint128,
    pub amount: Uint128,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    GetConfig {},
    #[returns(LiquidityResponse)]
    GetLiquidity { address: Addr },
}
