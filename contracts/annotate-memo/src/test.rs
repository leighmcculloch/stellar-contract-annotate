#![cfg(test)]

use super::*;
use soroban_sdk::{
    map, symbol_short,
    testutils::{Address as _, Events},
    token::{StellarAssetClient, TokenClient},
    vec, Env, IntoVal,
};

#[test]
fn test() {
    let env = Env::default();

    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let asset_admin = Address::generate(&env);
    let asset = env.register_stellar_asset_contract(asset_admin.clone());
    let asset_client = TokenClient::new(&env, &asset);
    let asset_admin_client = StellarAssetClient::new(&env, &asset);

    let asset_name = asset_client.name();

    let from = Address::generate(&env);
    let to = Address::generate(&env);

    asset_admin_client.mock_all_auths().mint(&from, &100i128);

    // Call the transfer function wrapped in a contract call that publishes an
    // annotation event.
    let _ = client.mock_all_auths().annotate(
        &vec![
            &env,
            MemoAnnotation {
                auther: from.clone(),
                address: to.clone(),
                memo: "123456".into_val(&env),
            },
        ],
        &asset,
        &symbol_short!("transfer"),
        &(&from, &to, 50i128).into_val(&env),
    );

    assert_eq!(
        env.events()
            .all()
            // Skip the first two events which are for the set_admin and mint calls
            // that are a part of asset and test setup.
            .slice(2..),
        vec![
            &env,
            // Expect 2 events, the first being the annotation indicating that
            // the sender (from) is stating that in the context of trnsfrdst (a
            // transfer destination) the destination address has an associated
            // memo of "123456".
            (
                contract_id.clone(),
                // Expect these event topics.
                (symbol_short!("annotate"),).into_val(&env),
                // Expect this event body.
                map![
                    &env,
                    (symbol_short!("auther"), from.clone().to_val()),
                    (symbol_short!("address"), to.clone().to_val()),
                    (symbol_short!("memo"), "123456".into_val(&env)),
                ]
                .into_val(&env),
            ),
            // The second event is the event generated by the transfer function,
            // that knows nothing about the memo.
            (
                asset,
                // Expect these event topics.
                (symbol_short!("transfer"), &from, &to, asset_name).into_val(&env),
                // Expect this event body.
                50i128.into_val(&env)
            ),
        ],
    );
}
