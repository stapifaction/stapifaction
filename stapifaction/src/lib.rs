mod persistable;

#[cfg(feature = "json")]
pub mod json;

pub use persistable::{Persistable, Subset};

#[cfg(feature = "derive")]
pub use stapifaction_derive::Persistable;

pub mod serde {
    pub use erased_serde::Serialize as ErasedSerialize;
    pub use serde::{ser::SerializeStruct, Serialize, Serializer};
}
