#![no_std]

mod errors;
mod events;
mod storage;
mod types;

#[cfg(test)]
mod test;

pub use errors::ContractError;
pub use types::{DataKey, InvoiceRecord, InvoiceStatus};

use soroban_sdk::{contract, contractimpl, token, Address, BytesN, Env};

#[contract]
pub struct InvoiceRegistry;

#[contractimpl]
impl InvoiceRegistry {
    /// Initialize the contract with an administrative address.
    ///
    /// Can only be called once. Returns `ContractError::AlreadyInitialized` on subsequent calls.
    pub fn initialize(env: Env, admin: Address) -> Result<(), ContractError> {
        if storage::has_admin(&env) {
            return Err(ContractError::AlreadyInitialized);
        }
        storage::set_admin(&env, &admin);
        events::emit_initialized(&env, &admin);
        Ok(())
    }

    /// Register a new invoice commitment on-chain.
    ///
    /// # Authorization
    /// Requires `issuer.require_auth()`.
    ///
    /// # Errors
    /// - `ContractError::InvalidAmount`: if `amount <= 0`.
    /// - `ContractError::InvoiceAlreadyExists`: if an invoice with `invoice_id` already exists.
    pub fn register_invoice(
        env: Env,
        issuer: Address,
        invoice_id: BytesN<32>,
        amount: i128,
        token: Address,
        content_hash: BytesN<32>,
    ) -> Result<(), ContractError> {
        issuer.require_auth();

        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        if storage::has_invoice(&env, &invoice_id) {
            return Err(ContractError::InvoiceAlreadyExists);
        }

        let empty_ref = BytesN::from_array(&env, &[0u8; 32]);
        let record = InvoiceRecord {
            issuer: issuer.clone(),
            amount,
            token: token.clone(),
            content_hash: content_hash.clone(),
            status: InvoiceStatus::Unpaid,
            payment_ref: empty_ref,
            settled_at: 0,
        };

        storage::set_invoice(&env, &invoice_id, &record);
        events::emit_invoice_registered(&env, &issuer, &invoice_id, amount, &token, &content_hash);

        Ok(())
    }

    /// Settle invoice on-chain by transferring token from payer to issuer.
    ///
    /// # Authorization
    /// Requires `payer.require_auth()`.
    ///
    /// # Errors
    /// - `ContractError::InvoiceNotFound`: if `invoice_id` is not registered.
    /// - `ContractError::AlreadyPaid`: if invoice status is already `Paid`.
    /// - `ContractError::AlreadyCancelled`: if invoice status is `Cancelled`.
    pub fn pay_invoice(
        env: Env,
        payer: Address,
        invoice_id: BytesN<32>,
    ) -> Result<(), ContractError> {
        payer.require_auth();

        let mut record =
            storage::get_invoice(&env, &invoice_id).ok_or(ContractError::InvoiceNotFound)?;

        match record.status {
            InvoiceStatus::Paid => return Err(ContractError::AlreadyPaid),
            InvoiceStatus::Cancelled => return Err(ContractError::AlreadyCancelled),
            InvoiceStatus::Unpaid => (),
        }

        // Execute token transfer from payer to invoice issuer
        let token_client = token::Client::new(&env, &record.token);
        token_client.transfer(&payer, &record.issuer, &record.amount);

        let settled_at = env.ledger().timestamp();
        record.status = InvoiceStatus::Paid;
        record.payment_ref = invoice_id.clone();
        record.settled_at = settled_at;

        storage::set_invoice(&env, &invoice_id, &record);
        events::emit_invoice_paid(&env, &invoice_id, &payer, record.amount, settled_at);

        Ok(())
    }

    /// Settle invoice on-chain after verified direct Stellar L1 payment.
    ///
    /// # Authorization
    /// Requires `issuer.require_auth()`.
    ///
    /// # Errors
    /// - `ContractError::InvoiceNotFound`: if `invoice_id` is not registered.
    /// - `ContractError::Unauthorized`: if caller is not the invoice issuer.
    /// - `ContractError::AlreadyPaid`: if invoice is already marked `Paid`.
    /// - `ContractError::AlreadyCancelled`: if invoice is `Cancelled`.
    pub fn record_direct_settlement(
        env: Env,
        issuer: Address,
        invoice_id: BytesN<32>,
        payment_ref: BytesN<32>,
    ) -> Result<(), ContractError> {
        issuer.require_auth();

        let mut record =
            storage::get_invoice(&env, &invoice_id).ok_or(ContractError::InvoiceNotFound)?;

        if record.issuer != issuer {
            return Err(ContractError::Unauthorized);
        }

        match record.status {
            InvoiceStatus::Paid => return Err(ContractError::AlreadyPaid),
            InvoiceStatus::Cancelled => return Err(ContractError::AlreadyCancelled),
            InvoiceStatus::Unpaid => (),
        }

        let settled_at = env.ledger().timestamp();
        record.status = InvoiceStatus::Paid;
        record.payment_ref = payment_ref.clone();
        record.settled_at = settled_at;

        storage::set_invoice(&env, &invoice_id, &record);
        events::emit_direct_settlement_recorded(
            &env,
            &invoice_id,
            &issuer,
            &payment_ref,
            settled_at,
        );

        Ok(())
    }

    /// Cancel an unpaid invoice commitment.
    ///
    /// # Authorization
    /// Requires `issuer.require_auth()`.
    ///
    /// # Errors
    /// - `ContractError::InvoiceNotFound`: if `invoice_id` is not registered.
    /// - `ContractError::Unauthorized`: if caller is not the invoice issuer.
    /// - `ContractError::AlreadyPaid`: if invoice has already been settled.
    /// - `ContractError::AlreadyCancelled`: if invoice is already cancelled.
    pub fn cancel_invoice(
        env: Env,
        issuer: Address,
        invoice_id: BytesN<32>,
    ) -> Result<(), ContractError> {
        issuer.require_auth();

        let mut record =
            storage::get_invoice(&env, &invoice_id).ok_or(ContractError::InvoiceNotFound)?;

        if record.issuer != issuer {
            return Err(ContractError::Unauthorized);
        }

        match record.status {
            InvoiceStatus::Paid => return Err(ContractError::AlreadyPaid),
            InvoiceStatus::Cancelled => return Err(ContractError::AlreadyCancelled),
            InvoiceStatus::Unpaid => (),
        }

        record.status = InvoiceStatus::Cancelled;
        storage::set_invoice(&env, &invoice_id, &record);
        events::emit_invoice_cancelled(&env, &invoice_id, &issuer);

        Ok(())
    }

    /// Query an on-chain invoice record.
    pub fn get_invoice(env: Env, invoice_id: BytesN<32>) -> Option<InvoiceRecord> {
        storage::get_invoice(&env, &invoice_id)
    }

    /// Query the configured admin address.
    pub fn get_admin(env: Env) -> Option<Address> {
        storage::get_admin(&env)
    }
}
