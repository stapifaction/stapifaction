mod expand_strategy;
mod persistable;
mod persister;
mod resolvable_path;

pub use expand_strategy::ExpandStrategy;
pub use persistable::{Child, Persistable};
pub use persister::*;
pub use resolvable_path::ResolvablePath;

#[cfg(feature = "derive")]
pub use stapifaction_derive::Persistable;

pub mod serde {
    pub use erased_serde::Serialize as ErasedSerialize;
    pub use serde::{ser::SerializeStruct, Serialize, Serializer};
}
