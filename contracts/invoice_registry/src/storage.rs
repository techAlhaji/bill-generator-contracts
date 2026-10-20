use soroban_sdk::{Address, BytesN, Env};

use crate::types::{DataKey, InvoiceRecord};

/// Number of ledgers (~1 day at ~5s per ledger) before TTL extension triggers.
pub const LEDGER_THRESHOLD: u32 = 17_280;

/// Target number of ledgers (~30 days) to extend the entry's lifetime to.
pub const LEDGER_EXTEND_TO: u32 = 518_400;

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
    env.storage()
        .instance()
        .extend_ttl(LEDGER_THRESHOLD, LEDGER_EXTEND_TO);
}

pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage()
        .instance()
        .extend_ttl(LEDGER_THRESHOLD, LEDGER_EXTEND_TO);
    env.storage().instance().get(&DataKey::Admin)
}

pub fn has_invoice(env: &Env, invoice_id: &BytesN<32>) -> bool {
    let key = DataKey::Invoice(invoice_id.clone());
    env.storage().persistent().has(&key)
}

pub fn get_invoice(env: &Env, invoice_id: &BytesN<32>) -> Option<InvoiceRecord> {
    let key = DataKey::Invoice(invoice_id.clone());
    if env.storage().persistent().has(&key) {
        env.storage()
            .persistent()
            .extend_ttl(&key, LEDGER_THRESHOLD, LEDGER_EXTEND_TO);
        env.storage().persistent().get(&key)
    } else {
        None
    }
}

pub fn set_invoice(env: &Env, invoice_id: &BytesN<32>, record: &InvoiceRecord) {
    let key = DataKey::Invoice(invoice_id.clone());
    env.storage().persistent().set(&key, record);
    env.storage()
        .persistent()
        .extend_ttl(&key, LEDGER_THRESHOLD, LEDGER_EXTEND_TO);
}
