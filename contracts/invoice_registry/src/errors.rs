use soroban_sdk::contracterror;

/// Canonical error codes returned by the InvoiceRegistry contract.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    Unauthorized = 2,
    InvalidAmount = 3,
    InvoiceAlreadyExists = 4,
    InvoiceNotFound = 5,
    AlreadyPaid = 6,
    AlreadyCancelled = 7,
    TransferFailed = 8,
}
