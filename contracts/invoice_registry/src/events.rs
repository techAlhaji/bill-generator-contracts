use soroban_sdk::{Address, BytesN, Env, Symbol};

pub fn emit_initialized(env: &Env, admin: &Address) {
    let topics = (Symbol::new(env, "init"), admin.clone());
    env.events().publish(topics, ());
}

pub fn emit_invoice_registered(
    env: &Env,
    issuer: &Address,
    invoice_id: &BytesN<32>,
    amount: i128,
    token: &Address,
    content_hash: &BytesN<32>,
) {
    let topics = (
        Symbol::new(env, "register"),
        issuer.clone(),
        invoice_id.clone(),
    );
    let data = (amount, token.clone(), content_hash.clone());
    env.events().publish(topics, data);
}

pub fn emit_invoice_paid(
    env: &Env,
    invoice_id: &BytesN<32>,
    payer: &Address,
    amount: i128,
    settled_at: u64,
) {
    let topics = (Symbol::new(env, "pay"), invoice_id.clone(), payer.clone());
    let data = (amount, settled_at);
    env.events().publish(topics, data);
}

pub fn emit_direct_settlement_recorded(
    env: &Env,
    invoice_id: &BytesN<32>,
    issuer: &Address,
    payment_ref: &BytesN<32>,
    settled_at: u64,
) {
    let topics = (
        Symbol::new(env, "settle_direct"),
        invoice_id.clone(),
        issuer.clone(),
    );
    let data = (payment_ref.clone(), settled_at);
    env.events().publish(topics, data);
}

pub fn emit_invoice_cancelled(env: &Env, invoice_id: &BytesN<32>, issuer: &Address) {
    let topics = (
        Symbol::new(env, "cancel"),
        invoice_id.clone(),
        issuer.clone(),
    );
    env.events().publish(topics, ());
}
