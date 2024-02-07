use cosmwasm_std::{
    entry_point,
    from_binary,
    to_binary,
    Addr,
    Binary,
    Deps,
    DepsMut,
    Env,
    MessageInfo,
    Order,
    Response,
    StdResult,
    Uint128,
};
use cw20::Cw20ReceiveMsg;

use crate::error::ContractError;
use crate::msg::{
    ConfigResponse,
    ExecuteMsg,
    InstantiateMsg,
    LiquiditiesResponse,
    LiquidityReceiveMsg,
    QueryMsg,
};
use crate::state::{ Config, LiquidityPool, CONFIG, LP_MAP };
use crate::util;

use cw2::set_contract_version;

const CONTRACT_NAME: &str = "Liquidity Locker";
const CONTRACT_VERSION: &str = "1.0";
const ONE_MONTH: u64 = 2629800u64;
const THREE_MONTH: u64 = 7889400u64;
const SIX_MONTH: u64 = 15778800u64;
const ONE_YEAR: u64 = 31557600u64;
const TWO_YEAR: u64 = 63115200u64;
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        owner: msg.owner.clone(),
        creator: msg.owner.clone(),
        fees_percentage: 1u64,
        fee_address: msg.fee_address.clone(),
        enabled: true,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig { native_token, fee_address, fees_percentage, is_enabled } =>
            util::execute_update_config(
                deps.storage,
                info.sender,
                native_token,
                fee_address,
                fees_percentage,
                is_enabled
            ),
        ExecuteMsg::Receive(msg) => execute_receive_liquidity(deps, env, info, msg),
        ExecuteMsg::Unstake { denom } => execute_unstake(deps, env, info, denom),
    }
}

fn execute_receive_liquidity(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    wrapper: Cw20ReceiveMsg
) -> Result<Response, ContractError> {
    let msg: LiquidityReceiveMsg = from_binary(&wrapper.msg)?;
    let cfg = CONFIG.load(deps.storage)?;
    if !cfg.enabled {
        return Err(ContractError::Disabled {});
    }
    match msg {
        LiquidityReceiveMsg::Lock { id, locktime } => {
            if
                locktime != ONE_MONTH &&
                locktime != THREE_MONTH &&
                locktime != SIX_MONTH &&
                locktime != ONE_YEAR &&
                locktime != TWO_YEAR
            {
                return Err(ContractError::LockedPeriodWrong {});
            }
            let amount = wrapper.amount;
            let denom = info.sender.clone();
            let owner = deps.api.addr_validate(&wrapper.sender.clone())?;
            let fee = cfg.fees_percentage;
            let fee_amount = (amount * Uint128::from(fee)) / Uint128::from(100u64);
            let new_amount = amount - fee_amount;
            if id.is_some() {
                let exists = LP_MAP.load(deps.storage, id.unwrap());
                match exists {
                    Ok(mut lp_pool) => {
                        if lp_pool.owner != owner {
                            return Err(ContractError::InvalidOwner {});
                        }
                        lp_pool.amount += new_amount;
                        lp_pool.locktime += locktime;
                        LP_MAP.save(deps.storage, lp_pool.clone().id, &lp_pool)?;
                        let fee_msg = util::transfer_token_message(
                            denom.to_string().clone(),
                            "cw20".to_string(),
                            fee_amount,
                            cfg.fee_address
                        )?;
                        return Ok(
                            Response::default()
                                .add_message(fee_msg)
                                .add_attribute("action", "lock_liquidity_pool")
                                .add_attribute("lp_owner", owner)
                                .add_attribute("lp_denom", denom)
                                .add_attribute("lp_amount", new_amount)
                                .add_attribute("locktime", locktime.to_string())
                        );
                    }
                    Err(_) => {
                        return Err(ContractError::NoLPFound {});
                    }
                }
            }

            let id = format!("{}-{}-{:x}", owner.clone(), denom.clone(), env.block.time.seconds());
            if LP_MAP.load(deps.storage, id.clone()).is_ok() {
                return Err(ContractError::InvalidID {});
            }

            let lp = LiquidityPool {
                id: id.clone(),
                owner: owner.clone(),
                denom: denom.to_string().clone(),
                locktime: env.block.time.seconds() + locktime,
                amount: new_amount,
            };

            LP_MAP.save(deps.storage, id.clone(), &lp)?;

            let fee_msg = util::transfer_token_message(
                denom.to_string().clone(),
                "cw20".to_string(),
                fee_amount,
                cfg.fee_address
            )?;
            Ok(
                Response::default()
                    .add_message(fee_msg)
                    .add_attribute("action", "lock_liquidity_pool")
                    .add_attribute("lp_owner", owner)
                    .add_attribute("lp_denom", denom)
                    .add_attribute("lp_amount", amount)
                    .add_attribute("locktime", locktime.to_string())
            )
        }
    }
}

fn execute_unstake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String
) -> Result<Response, ContractError> {
    let found = LP_MAP.load(deps.storage, id.clone());
    let cfg = CONFIG.load(deps.storage)?;
    if !cfg.enabled {
        return Err(ContractError::Disabled {});
    }
    match found {
        Ok(lp) => {
            let current_time = env.block.time.seconds();

            if current_time < lp.locktime {
                return Err(ContractError::Locktime {});
            }

            let msg = util::transfer_token_message(
                lp.denom.clone(),
                "cw20".to_string(),
                lp.amount,
                info.sender.clone()
            )?;

            LP_MAP.remove(deps.storage, id.clone());
            Ok(
                Response::default()
                    .add_attribute("action", "execute_unstake")
                    .add_attribute("lp_id", id.clone())
                    .add_attribute("amount", lp.amount)
                    .add_message(msg)
            )
        }
        Err(_) => Err(ContractError::NoLPFound {}),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
        QueryMsg::GetLiquidity { id } => to_binary(&query_liquidity(deps, id)?),
        QueryMsg::GetLiquidities { address } => to_binary(&query_liquidities(deps, address)?),
    }
}

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config: Config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner.clone(),
        enabled: true,
        fees_percentage: 1u64,
    })
}

pub fn query_liquidities(deps: Deps, address: Option<Addr>) -> StdResult<LiquiditiesResponse> {
    let liquidities: StdResult<Vec<LiquidityPool>> = LP_MAP.range(
        deps.storage,
        None,
        None,
        Order::Ascending
    )
        .map(|item| item.map(|(_, v)| v))
        .collect();

    match liquidities {
        Ok(mut lps) => {
            if address.is_some() {
                let unwrapped_address = address.unwrap();
                lps = lps
                    .into_iter()
                    .filter(|lp| { lp.owner == unwrapped_address })
                    .collect();
            }
            Ok(LiquiditiesResponse { liquidities: lps })
        }
        Err(_) => Ok(LiquiditiesResponse { liquidities: Vec::new() }),
    }
}

pub fn query_liquidity(deps: Deps, id: String) -> StdResult<LiquidityPool> {
    let liquidity = LP_MAP.load(deps.storage, id.clone())?;
    Ok(liquidity)
}
