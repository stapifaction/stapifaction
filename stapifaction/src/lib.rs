#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    unreachable_pub,
    clippy::missing_const_for_fn,
    rustdoc::all
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod path_element;
mod path_style;
mod persist;
mod persister;
mod resolvable_path;

pub use path_element::PathElement;
pub use path_style::PathStyle;
pub use persist::{Child, Persist};
pub use persister::*;
pub use resolvable_path::ResolvablePath;

#[cfg(feature = "derive")]
pub use stapifaction_derive::Persist;

pub use erased_serde::Serialize as ErasedSerialize;
