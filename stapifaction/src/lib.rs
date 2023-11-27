mod persistable;
mod persister;

pub use persistable::Persistable;
pub use persister::Persister;

#[cfg(feature = "derive")]
pub use stapifaction_derive::Persistable;

pub mod serde {
    pub use erased_serde::Serialize as ErasedSerialize;
    pub use serde::{ser::SerializeStruct, Serialize, Serializer};
}

