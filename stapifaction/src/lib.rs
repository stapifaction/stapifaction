#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    unreachable_pub,
    clippy::missing_const_for_fn,
    rustdoc::all
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod expand_strategy;
mod path_element;
mod persist;
mod persister;
mod resolvable_path;

pub use expand_strategy::ExpandStrategy;
pub use path_element::PathElement;
pub use persist::{Child, Persist};
pub use persister::*;
pub use resolvable_path::ResolvablePath;

#[cfg(feature = "derive")]
pub use stapifaction_derive::Persist;

pub use erased_serde::Serialize as ErasedSerialize;
