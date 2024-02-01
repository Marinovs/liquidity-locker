use cosmwasm_std::{
    to_binary,
    Addr,
    BalanceResponse as NativeBalanceResponse,
    BankMsg,
    BankQuery,
    Coin,
    CosmosMsg,
    QuerierWrapper,
    QueryRequest,
    Response,
    StdResult,
    Storage,
    Uint128,
    WasmMsg,
    WasmQuery,
};
use cw20::{ BalanceResponse as CW20BalanceResponse, Cw20ExecuteMsg, Cw20QueryMsg };

use crate::{ state::CONFIG, ContractError };

pub fn check_owner(storage: &mut dyn Storage, address: Addr) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(storage)?;

    if address != cfg.owner && address != cfg.creator {
        return Err(ContractError::Unauthorized {});
    }
    Ok(Response::new().add_attribute("action", "check_owner"))
}

#[allow(clippy::too_many_arguments)]
pub fn execute_update_config(
    storage: &mut dyn Storage,
    address: Addr,
    native_token: String,
    fee_address: Addr,
    fees_percentage: u64,
    is_enabled: bool
) -> Result<Response, ContractError> {
    check_owner(storage, address)?;

    CONFIG.update(
        storage,
        |mut exists| -> StdResult<_> {
            exists.fee_address = fee_address.clone();
            exists.fees_percentage = fees_percentage;
            exists.enabled = is_enabled;
            Ok(exists)
        }
    )?;

    Ok(
        Response::new()
            .add_attribute("action", "update_config")
            .add_attribute("native_token", native_token.clone())
            .add_attribute("fee_address", fee_address.clone())
            .add_attribute("fees_percentage", fees_percentage.to_string().clone())
    )
}

pub fn transfer_token_message(
    denom: String,
    token_type: String,
    amount: Uint128,
    receiver: Addr
) -> Result<CosmosMsg, ContractError> {
    if token_type == "native" {
        Ok(
            (BankMsg::Send {
                to_address: receiver.clone().into(),
                amount: vec![Coin {
                    denom: denom.clone(),
                    amount,
                }],
            }).into()
        )
    } else {
        Ok(
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: denom.clone(),
                funds: vec![],
                msg: to_binary(
                    &(Cw20ExecuteMsg::Transfer {
                        recipient: receiver.clone().into(),
                        amount,
                    })
                )?,
            })
        )
    }
}

pub fn get_token_amount(
    querier: QuerierWrapper,
    denom: String,
    contract_addr: Addr,
    token_type: String
) -> Result<Uint128, ContractError> {
    if token_type == "native" {
        let native_response: NativeBalanceResponse = querier.query(
            &QueryRequest::Bank(BankQuery::Balance {
                address: contract_addr.clone().into(),
                denom: denom.clone(),
            })
        )?;
        Ok(native_response.amount.amount)
    } else {
        let balance_response: CW20BalanceResponse = querier.query(
            &QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: denom.clone(),
                msg: to_binary(
                    &(Cw20QueryMsg::Balance {
                        address: contract_addr.clone().into(),
                    })
                )?,
            })
        )?;
        Ok(balance_response.balance)
    }
}
