//! events api
use crate::{
    api::{config::GearConfig, generated::api::Event, Api},
    Result,
};
use subxt::{
    events::EventSubscription,
    rpc::Subscription,
    sp_runtime::{generic::Header, traits::BlakeTwo256},
    TransactionEvents,
};

type Events<'a> = EventSubscription<'a, Subscription<Header<u32, BlakeTwo256>>, GearConfig, Event>;

/// transaction events
pub type InBlockEvents<'client> = TransactionEvents<'client, GearConfig, Event>;

impl Api {
    /// Subscribe all events
    #[allow(unused)]
    pub async fn events(&self) -> Result<Events<'_>> {
        Ok(self.runtime.events().subscribe().await?)
    }
}
