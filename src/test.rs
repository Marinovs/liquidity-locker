#[cfg(test)]
mod test_module {
    use cosmwasm_std::{ Addr, Env };
    use cosmwasm_std::testing::{ mock_dependencies, mock_env, mock_info };
    use time::OffsetDateTime;
    use chrono::prelude::*;

    // use injective_std::types::cosmos::bank::v1beta1::{ MsgSend, QueryBalanceRequest };
    // use crate::contract::{ execute, instantiate, query };
    use crate::error::ContractError;
    use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg, CollectionResponse };
    use cosmwasm_std::{ Coin, BankMsg, Uint128 };
    // use injective_test_tube::{ Account, Module, InjectiveTestApp, Wasm, Bank };
    // use injective_std::types::cosmos::bank::v1beta1::{ QueryBalanceRequest, MsgSend };
    // use injective_std::types::cosmos::base::v1beta1::Coin as BaseCoin;
    use std::result::Result;
    // #[test]
    // fn testSendToken() -> Result<(), ContractError> {
    //     let app = InjectiveTestApp::new();
    //     let accs = app
    //         .init_accounts(
    //             &[
    //                 Coin::new(1_000_000_000_000, "usdt"),
    //                 Coin::new(1_000_000_000_000, "inj"),
    //                 Coin::new(100, "a"),
    //             ],
    //             2
    //         )
    //         .unwrap();
    //     let admin = &accs[0];
    //     let new_admin = &accs[1];

    //     let bank = Bank::new(&app);
    //     let admin_response = bank
    //         .query_balance(
    //             &(QueryBalanceRequest {
    //                 address: admin.address(),
    //                 denom: "factory/inj1mldpx3uh7jx25cr7wd4c7g6gwda7wa7mfnq469/injscoin".to_string(),
    //             })
    //         )
    //         .unwrap();

    //     let admin_balance_result = admin_response.balance.unwrap().amount.parse();
    //     let denom = admin_response.balance.unwrap().denom;
    //     match admin_balance_result {
    //         Ok(admin_balance) => {
    //             println!("{:?}", admin_balance);
    //             const TOKEN_AMOUNT: &str = "10";
    //             let token_amount: Uint128 = Uint128::from(10u128);

    //             if admin_balance < token_amount {
    //                 return Err(crate::ContractError::Unauthorized {});
    //             }

    //             bank.send(
    //                 MsgSend {
    //                     from_address: admin.address(),
    //                     to_address: new_admin.address(),
    //                     amount: vec![BaseCoin {
    //                         amount: token_amount.to_string(),
    //                         denom: "factory/inj1mldpx3uh7jx25cr7wd4c7g6gwda7wa7mfnq469/injscoin".to_string(),
    //                     }],
    //                 },
    //                 &admin
    //             ).unwrap();

    //             let new_admin_response = bank
    //                 .query_balance(
    //                     &(QueryBalanceRequest {
    //                         address: new_admin.address(),
    //                         denom: "factory/inj1mldpx3uh7jx25cr7wd4c7g6gwda7wa7mfnq469/injscoin".to_string(),
    //                     })
    //                 )
    //                 .unwrap();

    //             let new_admin_balance = new_admin_response.balance.unwrap();

    //             println!("{:} | {:} {:}", admin.address(), admin_balance.to_string(), denom);
    //             println!("{:} | {:} {:}", new_admin.address(), new_admin_balance.amount, denom);
    //         }
    //         Err(_) => {
    //             // Handle the parsing error here.
    //             // For example, return an error or panic.
    //             return Err(crate::ContractError::Unauthorized {});
    //         }
    //     }

    //     // let token_amount: Uint128 = TOKEN_AMOUNT.parse();

    //     // BankMsg::Send {
    //     //     to_address: new_admin.address().clone(),
    //     //     amount: vec![Coin {
    //     //         denom: "inj".to_string(),
    //     //         amount: token_amount,
    //     //     }],
    //     // };
    //     // }

    //     Ok(())
    // }

    #[test]
    // fn testExecute() {
    //     let app = InjectiveTestApp::new();
    //     let accs = app
    //         .init_accounts(
    //             &[Coin::new(1_000_000_000_000, "usdt"), Coin::new(1_000_000_000_000, "inj")],
    //             2
    //         )
    //         .unwrap();
    //     let admin = &accs[0];
    //     let new_admin = &accs[1];
    //     let wasm = Wasm::new(&app);

    //     let wasm_byte_code = std::fs::read("./artifacts/test-aarch64.wasm").unwrap();
    //     let code_id = wasm.store_code(&wasm_byte_code, None, admin).unwrap().data.code_id;
    //     let contract_addr = wasm
    //         .instantiate(
    //             code_id,
    //             &(InstantiateMsg {
    //                 owner: Addr::unchecked(admin.address()),
    //                 native_token: "inj".to_string(),
    //             }),
    //             None, // contract admin used for migration, not the same as cw1_whitelist admin
    //             None, // contract label
    //             &[], // funds
    //             admin // signer
    //         )
    //         .unwrap().data.address;

    //     let resp = wasm
    //         .execute::<ExecuteMsg>(
    //             &contract_addr,
    //             &(ExecuteMsg::AddCollection {
    //                 collection_address: Addr::unchecked(
    //                     "inj1257sqg3jnu2xdv9fyv2ffjd60fhjlutgnvtd4s".to_string()
    //                 ),
    //                 duration: "2584000".parse::<u64>().unwrap(),
    //                 fee_address: Addr::unchecked(admin.address()),
    //                 unstaking_fee: Uint128::from(500000000000000000u128),
    //                 reward_token: "factory/inj1mldpx3uh7jx25cr7wd4c7g6gwda7wa7mfnq469/injscoin".to_string(),
    //                 reward_cooldown: "60".parse::<u64>().unwrap(),
    //                 reward_amount: Uint128::from(10u128),
    //                 enabled: true,
    //             }),
    //             &[],
    //             admin
    //         )
    //         .unwrap();

    //     let collection = wasm
    //         .query::<QueryMsg, CollectionResponse>(
    //             &contract_addr,
    //             &(QueryMsg::GetCollection {
    //                 collection_address: Addr::unchecked(
    //                     "inj1257sqg3jnu2xdv9fyv2ffjd60fhjlutgnvtd4s".to_string()
    //                 ),
    //             })
    //         )
    //         .unwrap();
    // }

    #[test]
    fn testSeconds() {
        let current_time = mock_env().block.time.seconds();
        let current_ms = current_time * 1000;
        let claim_cooldown = 60;
        let now = OffsetDateTime::now_utc();
        let utc_now = Utc::now();
        let seconds_since_epoch = utc_now.timestamp();

        let next_cooldown = current_time + claim_cooldown;

        println!(
            "Now: {seconds_since_epoch}\nCurrent Time: {:}s\nCurrent Time (MS): {:}\nCooldown: {:}s\nNext Cooldown: {:}s\n",
            current_time,
            current_ms,
            claim_cooldown,
            next_cooldown
        );
    }
}
