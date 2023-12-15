mod path_resolve_strategy;
mod persistable;
mod persister;

pub use path_resolve_strategy::PathResolveStrategy;
pub use persistable::{Child, Persistable};
pub use persister::*;

#[cfg(feature = "derive")]
pub use stapifaction_derive::Persistable;

pub mod serde {
    pub use erased_serde::Serialize as ErasedSerialize;
    pub use serde::{ser::SerializeStruct, Serialize, Serializer};
}
