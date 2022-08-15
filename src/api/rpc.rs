//! gear api rpc methods
use crate::{
    api::{generated::api::runtime_types::gear_common::ActiveProgram, types, Api},
    result::{Error, Result},
};
use hex::ToHex;
use parity_scale_codec::Decode;
use std::{collections::HashMap, sync::Arc};
use subxt::{
    rpc::{rpc_params, ClientT},
    sp_core::{storage::StorageKey, Bytes, H256},
    RpcClient,
};

const GPROG: [u8; 9] = *b"g::prog::";
const GPAGES: [u8; 10] = *b"g::pages::";
const SEPARATOR: [u8; 2] = *b"::";

impl Api {
    /// get rpc client
    pub fn rpc(&self) -> Arc<RpcClient> {
        self.runtime.client.rpc().client.clone()
    }

    /// public key of the signer in H256
    pub fn source(&self) -> H256 {
        AsRef::<[u8; 32]>::as_ref(self.signer.account_id()).into()
    }

    /// gear_getInitGasSpent
    pub async fn get_init_gas_spent(
        &self,
        code: Bytes,
        payload: Bytes,
        value: u64,
        at: Option<H256>,
    ) -> Result<types::GasInfo> {
        self.rpc()
            .request(
                "gear_calculateInitGas",
                rpc_params![self.source(), code, payload, value, true, at],
            )
            .await
            .map_err(Into::into)
    }

    /// gear_getHandleGasSpent
    #[allow(dead_code)]
    pub async fn get_handle_gas_spent(
        &self,
        dest: H256,
        payload: Bytes,
        value: u128,
        at: Option<H256>,
    ) -> Result<types::GasInfo> {
        self.rpc()
            .request(
                "gear_calculateHandleGas",
                rpc_params![self.source(), dest, payload, value, true, at],
            )
            .await
            .map_err(Into::into)
    }

    /// Get active program from program id.
    pub async fn gprog(&self, pid: H256) -> Result<ActiveProgram> {
        let bytes = self
            .runtime
            .client
            .storage()
            .fetch_raw(StorageKey([GPROG.as_slice(), &pid.0].concat()), None)
            .await?
            .ok_or_else(|| Error::ProgramNotFound(pid.encode_hex()))?;

        Ok(ActiveProgram::decode(&mut bytes.0.as_ref())?)
    }

    /// Get pages of active program.
    pub async fn gpages(&self, pid: H256, program: ActiveProgram) -> Result<types::GearPages> {
        let mut pages = HashMap::new();
        let prefix = [GPAGES.as_slice(), &pid.0, &SEPARATOR].concat();
        for page in program.pages_with_data {
            let value = self
                .runtime
                .client
                .storage()
                .fetch_raw(
                    StorageKey([prefix.as_slice(), &page.0.to_le_bytes()].concat()),
                    None,
                )
                .await?
                .ok_or_else(|| Error::PageNotFound(page.0, pid.encode_hex()))?;
            pages.insert(page.0, value.0);
        }

        Ok(pages)
    }

    /// Get program pages from program id.
    pub async fn program_pages(&self, pid: H256) -> Result<types::GearPages> {
        self.gpages(pid, self.gprog(pid).await?).await
    }
}
