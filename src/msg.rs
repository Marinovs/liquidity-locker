use cosmwasm_schema::{ cw_serde, QueryResponses };
use cosmwasm_std::{ Addr, Uint128 };
use cw20::Cw20ReceiveMsg;

use crate::state::LiquidityPool;

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
        is_enabled: bool,
    },
    Receive(Cw20ReceiveMsg),
    Unstake {
        denom: String,
    },
}

#[cw_serde]
pub enum LiquidityReceiveMsg {
    Lock {
        id: Option<String>,
        locktime: u64,
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
pub struct LiquiditiesResponse {
    pub liquidities: Vec<LiquidityPool>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)] GetConfig {},
    #[returns(LiquiditiesResponse)] GetLiquidities {
        address: Option<Addr>,
    },
    #[returns(LiquidityResponse)] GetLiquidity {
        id: String,
    },
}
