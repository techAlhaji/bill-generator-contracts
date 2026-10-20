use soroban_sdk::{contracttype, Address, BytesN};

/// Status of an on-chain invoice commitment.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum InvoiceStatus {
    Unpaid = 0,
    Paid = 1,
    Cancelled = 2,
}

/// On-chain invoice record storing privacy-preserving commitments and settlement state.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvoiceRecord {
    pub issuer: Address,
    pub amount: i128,
    pub token: Address,
    pub content_hash: BytesN<32>,
    pub status: InvoiceStatus,
    pub payment_ref: BytesN<32>,
    pub settled_at: u64,
}

/// Storage keys for the contract.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Invoice(BytesN<32>),
}
