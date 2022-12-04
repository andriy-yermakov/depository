use near_units::parse_near;
use serde_json::json;
use std::{env, fs};
use workspaces::{Account, Contract};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let wasm_arg: &str = &(env::args().nth(1).unwrap());
    let wasm_filepath = fs::canonicalize(env::current_dir()?.join(wasm_arg))?;

    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(wasm_filepath)?;
    let contract = worker.dev_deploy(&wasm).await?;

    // create accounts
    let account = worker.dev_create_account().await?;
    let deposit_account = account
        .create_subaccount("deposit_account")
        .initial_balance(parse_near!("1 N"))
        .transact()
        .await?
        .into_result()?;

    let alice = account
        .create_subaccount("alice")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    // init contract
    contract
        .as_account()
        .call(contract.id(), "new")
        .args_json(json!({ "deposit_account": deposit_account.id() }))
        .transact()
        .await?
        .into_result()?;

    // begin tests
    test_deposit_odd(&alice, &contract).await?;
    test_deposit_even(&alice, &contract).await?;
    Ok(())
}

async fn test_deposit_odd(user: &Account, contract: &Contract) -> anyhow::Result<()> {
    let result: bool = user
        .call(contract.id(), "deposit")
        .args_json(json!({}))
        .deposit(1)
        .transact()
        .await?
        .json()?;

    assert_eq!(result, false);
    println!("      Passed ✅ deposit_odd");

    Ok(())
}

async fn test_deposit_even(user: &Account, contract: &Contract) -> anyhow::Result<()> {
    let result: bool = user
        .call(contract.id(), "deposit")
        .args_json(json!({}))
        .deposit(2)
        .transact()
        .await?
        .json()?;

    assert_eq!(result, true);
    println!("      Passed ✅ deposit_even");
    Ok(())
}
