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
    Response,
    StdResult,
    Uint128,
};
use cw20::Cw20ReceiveMsg;

use crate::error::ContractError;
use crate::msg::{ ConfigResponse, ExecuteMsg, InstantiateMsg, LiquidityReceiveMsg, QueryMsg };
use crate::state::{ Config, LiquidityPool, CONFIG, LP_MAP };
use crate::util;

use cw2::set_contract_version;

const CONTRACT_NAME: &str = "Liquidity Locker";
const CONTRACT_VERSION: &str = "1.0";
// const THREE_MONTH: u64 = 7889400u64;
const THREE_MONTH: u64 = 7889400u64;
const SIX_MONTH: u64 = 15778800u64;
const ONE_YEAR: u64 = 31557600u64;
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
        ExecuteMsg::UpdateConfig { native_token, fee_address, fees_percentage } =>
            util::execute_update_config(
                deps.storage,
                info.sender,
                native_token,
                fee_address,
                fees_percentage
            ),
        ExecuteMsg::Receive(msg) => execute_receive_liquidity(deps, env, info, msg),
        ExecuteMsg::Unstake { denom } => execute_unstake(deps, env, info, denom),
    }
}

fn execute_receive_liquidity(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    wrapper: Cw20ReceiveMsg
) -> Result<Response, ContractError> {
    let msg: LiquidityReceiveMsg = from_binary(&wrapper.msg)?;
    let cfg = CONFIG.load(deps.storage)?;
    match msg {
        LiquidityReceiveMsg::Lock { owner, denom, locktime, amount } => {
            if wrapper.amount != amount {
                return Err(ContractError::MissmatchedPayment {});
            }
            let exists = LP_MAP.load(deps.storage, owner.clone());
            let fee = cfg.fees_percentage;
            let fee_amount = (amount * Uint128::from(fee)) / Uint128::from(100u64);
            let new_amount = amount - fee_amount;
            match exists {
                Ok(mut lp_pool) => {
                    let index = lp_pool.iter().position(|x| x.denom == denom);
                    if index.is_some() {
                        let found = lp_pool.get(index.unwrap());
                        let mut unwraped = found.unwrap().clone();
                        unwraped.amount += new_amount;
                        unwraped.locktime += locktime;
                        lp_pool.remove(index.unwrap());
                        lp_pool.push(unwraped);
                        LP_MAP.save(deps.storage, owner.clone(), &lp_pool)?;
                    } else {
                        let lp = LiquidityPool {
                            owner: owner.clone(),
                            denom: denom.clone(),
                            amount: new_amount,
                            locktime: env.block.time.seconds() + locktime,
                        };
                        lp_pool.push(lp);
                        LP_MAP.save(deps.storage, owner.clone(), &lp_pool)?;
                    }

                    let fee_msg = util::transfer_token_message(
                        denom.clone(),
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
                            .add_attribute("lp_amount", new_amount)
                            .add_attribute("locktime", locktime.to_string())
                    )
                }

                Err(_) => {
                    if locktime != THREE_MONTH && locktime != SIX_MONTH && locktime != ONE_YEAR {
                        return Err(ContractError::LockedPeriodWrong {});
                    }

                    let lp = LiquidityPool {
                        owner: owner.clone(),
                        denom: denom.clone(),
                        locktime: env.block.time.seconds() + locktime,
                        amount: new_amount,
                    };

                    let lp_pool = vec![lp];
                    LP_MAP.save(deps.storage, owner.clone(), &lp_pool)?;

                    let fee_msg = util::transfer_token_message(
                        denom.clone(),
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
    }
}

fn execute_unstake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    denom: String
) -> Result<Response, ContractError> {
    let found = LP_MAP.load(deps.storage, info.sender.clone());

    match found {
        Ok(mut lp_pool) => {
            let index = lp_pool.iter().position(|x| x.denom == denom);
            if index.is_some() {
                let lp = lp_pool.get(index.unwrap()).unwrap();
                let current_time = env.block.time.seconds();

                if current_time < lp.locktime {
                    return Err(ContractError::Locktime {});
                }

                let msg = util::transfer_token_message(
                    denom.clone(),
                    "cw20".to_string(),
                    lp.amount,
                    info.sender.clone()
                )?;

                lp_pool.remove(index.unwrap());

                LP_MAP.save(deps.storage, info.sender.clone(), &lp_pool)?;

                Ok(Response::default().add_attribute("action", "execute_unstake").add_message(msg))
            } else {
                Err(ContractError::NoLPFound {})
            }
        }
        Err(_) => Err(ContractError::NoLPFound {}),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
        QueryMsg::GetLiquidity { address } => to_binary(&query_liquidity(deps, address)?),
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

pub fn query_liquidity(deps: Deps, address: Addr) -> StdResult<Vec<LiquidityPool>> {
    let liquidity = LP_MAP.load(deps.storage, address.clone())?;
    Ok(liquidity)
}
