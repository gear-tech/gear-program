//! events api
use crate::{
    api::{
        config::GearConfig,
        generated::api::{system::Event as SystemEvent, Event},
        Api,
    },
    Result,
};
use subxt::{
    events::EventSubscription,
    rpc::Subscription,
    sp_runtime::{generic::Header, traits::BlakeTwo256},
    TransactionEvents,
};

/// Generic events
pub type Events<'a> =
    EventSubscription<'a, Subscription<Header<u32, BlakeTwo256>>, GearConfig, Event>;

/// Transaction events
#[allow(unused)]
pub type InBlockEvents<'client> = TransactionEvents<'client, GearConfig, Event>;

impl Api {
    /// Subscribe all events
    #[allow(unused)]
    pub async fn events(&self) -> Result<Events<'_>> {
        Ok(self.runtime.events().subscribe().await?)
    }

    /// Parse transaction fee from InBlockEvents
    pub fn capture_weight_info(&self, events: InBlockEvents) -> Result<()> {
        for maybe_event in events.iter() {
            let event = maybe_event?.event;
            if let Event::System(SystemEvent::ExtrinsicSuccess { dispatch_info }) = event {
                println!("\tWeight spent: {:?}", dispatch_info.weight);
                return Ok(());
            }
        }

        Ok(())
    }
}
