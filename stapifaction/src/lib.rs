mod expand_strategy;
mod persistable;
mod persister;

pub use expand_strategy::ExpandStrategy;
pub use persistable::{Child, Persistable};
pub use persister::*;

#[cfg(feature = "derive")]
pub use stapifaction_derive::Persistable;

pub mod serde {
    pub use erased_serde::Serialize as ErasedSerialize;
    pub use serde::{ser::SerializeStruct, Serialize, Serializer};
}
