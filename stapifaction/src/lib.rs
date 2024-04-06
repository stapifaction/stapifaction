#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    unreachable_pub,
    clippy::missing_const_for_fn,
    rustdoc::all
)]

mod expand_strategy;
mod path_element;
mod persistable;
mod persister;
mod resolvable_path;

pub use expand_strategy::ExpandStrategy;
pub use path_element::PathElement;
pub use persistable::{Child, Persistable};
pub use persister::*;
pub use resolvable_path::ResolvablePath;

#[cfg(feature = "derive")]
pub use stapifaction_derive::Persistable;

pub use erased_serde::Serialize as ErasedSerialize;
