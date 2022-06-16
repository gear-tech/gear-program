//! gear storage apis
use crate::{api::Api, Result};
use subxt::{sp_core::crypto::Ss58Codec, sp_runtime::AccountId32};

impl Api {
    /// Get balance by account address
    pub async fn get_balance(&self, address: &str) -> Result<u128> {
        Ok(self
            .runtime
            .storage()
            .system()
            .account(&AccountId32::from_ss58check(address)?, None)
            .await?
            .data
            .free)
    }
}
