mod metadata;

pub use metadata::*;

use self::api::runtime_types::gear_runtime::RuntimeEvent;
use self::api::Event as ApiEvent;
use gear_core::message::ReplyDetails;
use gear_core::message::StoredMessage;
use gear_core::{ids, message};
use metadata::api::runtime_types::{
    gear_common::event::*, gear_core::ids as generated_ids, gear_core::message as generated_message,
};
use parity_scale_codec::{Decode, Encode};

impl From<ids::MessageId> for generated_ids::MessageId {
    fn from(other: ids::MessageId) -> Self {
        Self(other.into())
    }
}

impl From<generated_ids::MessageId> for ids::MessageId {
    fn from(other: generated_ids::MessageId) -> Self {
        other.0.into()
    }
}

impl From<ids::ProgramId> for generated_ids::ProgramId {
    fn from(other: ids::ProgramId) -> Self {
        Self(other.into())
    }
}

impl From<generated_ids::ProgramId> for ids::ProgramId {
    fn from(other: generated_ids::ProgramId) -> Self {
        other.0.into()
    }
}

impl From<ids::CodeId> for generated_ids::CodeId {
    fn from(other: ids::CodeId) -> Self {
        Self(other.into())
    }
}

impl From<generated_ids::CodeId> for ids::CodeId {
    fn from(other: generated_ids::CodeId) -> Self {
        other.0.into()
    }
}

impl From<generated_message::common::ReplyDetails> for message::ReplyDetails {
    fn from(other: generated_message::common::ReplyDetails) -> Self {
        ReplyDetails::new(other.reply_to.into(), other.exit_code)
    }
}

impl From<generated_message::stored::StoredMessage> for message::StoredMessage {
    fn from(other: generated_message::stored::StoredMessage) -> Self {
        StoredMessage::new(
            other.id.into(),
            other.source.into(),
            other.destination.into(),
            other.payload,
            other.value,
            other.reply.map(Into::into),
        )
    }
}

impl From<ApiEvent> for RuntimeEvent {
    fn from(ev: ApiEvent) -> Self {
        RuntimeEvent::decode(&mut ev.encode().as_ref()).expect("Infallible")
    }
}

impl From<RuntimeEvent> for ApiEvent {
    fn from(ev: RuntimeEvent) -> Self {
        ApiEvent::decode(&mut ev.encode().as_ref()).expect("Infallible")
    }
}

macro_rules! impl_basic {
    ($t:ty) => {
        impl Clone for $t {
            fn clone(&self) -> Self {
                Self::decode(&mut self.encode().as_ref()).expect("Infallible")
            }
        }

        impl PartialEq for $t {
            fn eq(&self, other: &Self) -> bool {
                self.encode().eq(&other.encode())
            }
        }
    };
    ($t:ty $(, $tt:ty) +) => {
        impl_basic!{ $t }
        $(impl_basic! { $tt }) +
    };
}

impl_basic! {
    ApiEvent, RuntimeEvent, generated_ids::MessageId,
    generated_ids::ProgramId, generated_ids::CodeId,
    Reason<UserMessageReadRuntimeReason, UserMessageReadSystemReason>
}
