//! gear storage apis
use crate::{
    api::{
        generated::api::runtime_types::{frame_system::AccountInfo, pallet_balances::AccountData},
        Api,
    },
    result::Result,
};
use subxt::{sp_core::crypto::Ss58Codec, sp_runtime::AccountId32};

impl Api {
    /// Get account info by address
    pub async fn info(&self, address: &str) -> Result<AccountInfo<u32, AccountData<u128>>> {
        Ok(self
            .runtime
            .storage()
            .system()
            .account(&AccountId32::from_ss58check(address)?, None)
            .await?)
    }

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

mod gear {
    use crate::{
        api::{
            generated::api::runtime_types::gear_core::{code::InstrumentedCode, ids::CodeId},
            Api,
        },
        result::Result,
    };

    impl Api {
        pub async fn code_storage(&self, code_hash: [u8; 32]) -> Result<Option<InstrumentedCode>> {
            Ok(self
                .runtime
                .storage()
                .gear_program()
                .code_storage(&CodeId(code_hash), None)
                .await?)
        }
    }
}
