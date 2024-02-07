use cosmwasm_schema::cw_serde;
use cosmwasm_std::{ Addr, Uint128 };
use cw_storage_plus::{ Item, Map };

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub creator: Addr,
    pub fees_percentage: u64,
    pub fee_address: Addr,
    pub enabled: bool,
}

#[cw_serde]
pub struct LiquidityPool {
    pub id: String,
    pub owner: Addr,
    pub denom: String,
    pub locktime: u64,
    pub amount: Uint128,
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const LP_MAP_PREFIX: &str = "lp_map";
pub const LP_MAP: Map<String, LiquidityPool> = Map::new(LP_MAP_PREFIX);
