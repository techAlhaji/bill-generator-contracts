#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token, Address, BytesN, Env,
};

fn setup_test<'a>() -> (
    Env,
    InvoiceRegistryClient<'a>,
    Address,
    Address,
    Address,
    token::Client<'a>,
    token::StellarAssetClient<'a>,
) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(InvoiceRegistry, ());
    let client = InvoiceRegistryClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let issuer = Address::generate(&env);
    let payer = Address::generate(&env);

    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_client = token::Client::new(&env, &token_contract.address());
    let token_asset_client = token::StellarAssetClient::new(&env, &token_contract.address());

    (
        env,
        client,
        admin,
        issuer,
        payer,
        token_client,
        token_asset_client,
    )
}

fn sample_invoice_id(env: &Env, val: u8) -> BytesN<32> {
    let mut bytes = [0u8; 32];
    bytes[0] = val;
    BytesN::from_array(env, &bytes)
}

fn sample_hash(env: &Env, val: u8) -> BytesN<32> {
    let mut bytes = [0xff; 32];
    bytes[0] = val;
    BytesN::from_array(env, &bytes)
}

#[test]
fn test_initialize_success_and_duplicate() {
    let (env, client, admin, _, _, _, _) = setup_test();

    assert_eq!(client.get_admin(), None);

    let res = client.try_initialize(&admin);
    assert!(res.is_ok());
    assert_eq!(client.get_admin(), Some(admin.clone()));

    // Attempt second initialization should fail
    let another_admin = Address::generate(&env);
    let res2 = client.try_initialize(&another_admin);
    assert_eq!(res2, Err(Ok(ContractError::AlreadyInitialized)));
}

#[test]
fn test_register_invoice_success() {
    let (env, client, _, issuer, _, token_client, _) = setup_test();

    let invoice_id = sample_invoice_id(&env, 1);
    let content_hash = sample_hash(&env, 1);
    let amount = 50_000_000i128; // 50 XLM in stroops

    let res = client.try_register_invoice(
        &issuer,
        &invoice_id,
        &amount,
        &token_client.address,
        &content_hash,
    );
    assert!(res.is_ok());

    let record = client
        .get_invoice(&invoice_id)
        .expect("Invoice should exist");
    assert_eq!(record.issuer, issuer);
    assert_eq!(record.amount, amount);
    assert_eq!(record.token, token_client.address);
    assert_eq!(record.content_hash, content_hash);
    assert_eq!(record.status, InvoiceStatus::Unpaid);
    assert_eq!(record.settled_at, 0);
    assert_eq!(record.payment_ref, BytesN::from_array(&env, &[0u8; 32]));
}

#[test]
fn test_register_duplicate_invoice_fails() {
    let (env, client, _, issuer, _, token_client, _) = setup_test();

    let invoice_id = sample_invoice_id(&env, 2);
    let content_hash = sample_hash(&env, 2);
    let amount = 10_000_000i128;

    let res1 = client.try_register_invoice(
        &issuer,
        &invoice_id,
        &amount,
        &token_client.address,
        &content_hash,
    );
    assert!(res1.is_ok());

    let res2 = client.try_register_invoice(
        &issuer,
        &invoice_id,
        &amount,
        &token_client.address,
        &content_hash,
    );
    assert_eq!(res2, Err(Ok(ContractError::InvoiceAlreadyExists)));
}

#[test]
fn test_register_invalid_amount_fails() {
    let (env, client, _, issuer, _, token_client, _) = setup_test();

    let invoice_id = sample_invoice_id(&env, 3);
    let content_hash = sample_hash(&env, 3);

    // Zero amount
    let res_zero = client.try_register_invoice(
        &issuer,
        &invoice_id,
        &0i128,
        &token_client.address,
        &content_hash,
    );
    assert_eq!(res_zero, Err(Ok(ContractError::InvalidAmount)));

    // Negative amount
    let res_neg = client.try_register_invoice(
        &issuer,
        &invoice_id,
        &-100i128,
        &token_client.address,
        &content_hash,
    );
    assert_eq!(res_neg, Err(Ok(ContractError::InvalidAmount)));
}

#[test]
fn test_pay_invoice_token_transfer_success() {
    let (env, client, _, issuer, payer, token_client, token_asset_client) = setup_test();

    let invoice_id = sample_invoice_id(&env, 4);
    let content_hash = sample_hash(&env, 4);
    let amount = 25_000_000i128;

    // Mint tokens to payer
    token_asset_client.mint(&payer, &100_000_000i128);
    assert_eq!(token_client.balance(&payer), 100_000_000i128);
    assert_eq!(token_client.balance(&issuer), 0i128);

    // Register invoice
    client.register_invoice(
        &issuer,
        &invoice_id,
        &amount,
        &token_client.address,
        &content_hash,
    );

    // Set ledger timestamp
    env.ledger().set_timestamp(1729375200);

    // Settle invoice on-chain
    let pay_res = client.try_pay_invoice(&payer, &invoice_id);
    assert!(pay_res.is_ok());

    // Verify token transfer balances
    assert_eq!(token_client.balance(&payer), 75_000_000i128);
    assert_eq!(token_client.balance(&issuer), 25_000_000i128);

    // Verify on-chain invoice record
    let record = client.get_invoice(&invoice_id).expect("Invoice must exist");
    assert_eq!(record.status, InvoiceStatus::Paid);
    assert_eq!(record.settled_at, 1729375200);
    assert_eq!(record.payment_ref, invoice_id);

    // Repaying should fail
    let repay_res = client.try_pay_invoice(&payer, &invoice_id);
    assert_eq!(repay_res, Err(Ok(ContractError::AlreadyPaid)));

    // Cancelling a paid invoice should fail
    let cancel_res = client.try_cancel_invoice(&issuer, &invoice_id);
    assert_eq!(cancel_res, Err(Ok(ContractError::AlreadyPaid)));
}

#[test]
fn test_record_direct_settlement_success() {
    let (env, client, _, issuer, stranger, token_client, _) = setup_test();

    let invoice_id = sample_invoice_id(&env, 5);
    let content_hash = sample_hash(&env, 5);
    let amount = 10_000_000i128;
    let payment_ref = sample_hash(&env, 0xbb);

    client.register_invoice(
        &issuer,
        &invoice_id,
        &amount,
        &token_client.address,
        &content_hash,
    );

    // Unauthorized non-issuer attempts to record direct settlement
    let unauth_res = client.try_record_direct_settlement(&stranger, &invoice_id, &payment_ref);
    assert_eq!(unauth_res, Err(Ok(ContractError::Unauthorized)));

    // Issuer records verified direct settlement
    env.ledger().set_timestamp(1729376000);
    let res = client.try_record_direct_settlement(&issuer, &invoice_id, &payment_ref);
    assert!(res.is_ok());

    let record = client.get_invoice(&invoice_id).expect("Invoice must exist");
    assert_eq!(record.status, InvoiceStatus::Paid);
    assert_eq!(record.settled_at, 1729376000);
    assert_eq!(record.payment_ref, payment_ref);

    // Re-settling should fail
    let re_settle = client.try_record_direct_settlement(&issuer, &invoice_id, &payment_ref);
    assert_eq!(re_settle, Err(Ok(ContractError::AlreadyPaid)));
}

#[test]
fn test_cancel_invoice_lifecycle() {
    let (env, client, _, issuer, stranger, token_client, _) = setup_test();

    let invoice_id = sample_invoice_id(&env, 6);
    let content_hash = sample_hash(&env, 6);
    let amount = 10_000_000i128;

    client.register_invoice(
        &issuer,
        &invoice_id,
        &amount,
        &token_client.address,
        &content_hash,
    );

    // Non-issuer cancellation fails
    let unauth_res = client.try_cancel_invoice(&stranger, &invoice_id);
    assert_eq!(unauth_res, Err(Ok(ContractError::Unauthorized)));

    // Issuer cancels
    let cancel_res = client.try_cancel_invoice(&issuer, &invoice_id);
    assert!(cancel_res.is_ok());

    let record = client.get_invoice(&invoice_id).expect("Invoice must exist");
    assert_eq!(record.status, InvoiceStatus::Cancelled);

    // Paying cancelled invoice fails
    let pay_res = client.try_pay_invoice(&stranger, &invoice_id);
    assert_eq!(pay_res, Err(Ok(ContractError::AlreadyCancelled)));

    // Re-cancelling fails
    let recancel_res = client.try_cancel_invoice(&issuer, &invoice_id);
    assert_eq!(recancel_res, Err(Ok(ContractError::AlreadyCancelled)));

    // Direct settlement on cancelled invoice fails
    let payment_ref = sample_hash(&env, 0xcc);
    let direct_res = client.try_record_direct_settlement(&issuer, &invoice_id, &payment_ref);
    assert_eq!(direct_res, Err(Ok(ContractError::AlreadyCancelled)));
}

#[test]
fn test_non_existent_invoice_operations_fail() {
    let (env, client, _, issuer, payer, _, _) = setup_test();

    let missing_id = sample_invoice_id(&env, 99);
    let payment_ref = sample_hash(&env, 99);

    assert_eq!(client.get_invoice(&missing_id), None);

    let pay_res = client.try_pay_invoice(&payer, &missing_id);
    assert_eq!(pay_res, Err(Ok(ContractError::InvoiceNotFound)));

    let cancel_res = client.try_cancel_invoice(&issuer, &missing_id);
    assert_eq!(cancel_res, Err(Ok(ContractError::InvoiceNotFound)));

    let direct_res = client.try_record_direct_settlement(&issuer, &missing_id, &payment_ref);
    assert_eq!(direct_res, Err(Ok(ContractError::InvoiceNotFound)));
}
