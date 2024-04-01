use std::path::PathBuf;

use crate::{PathElement, ResolvablePath};

/// An expand strategy defines how.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ExpandStrategy {
    /// Persists each child in a different folder.
    SeparateFolders(String),
    /// Persists each child in the same folder.
    UniqueFolder(String),
}

impl ExpandStrategy {
    /// Resolves a [`ResolvablePath`] to a [`PathBuf`].
    pub fn resolve_path(&self, resolvable: &ResolvablePath, child_count: usize) -> PathBuf {
        let path = PathBuf::from(resolvable);
        match self {
            ExpandStrategy::SeparateFolders(name) => path.join(name),
            ExpandStrategy::UniqueFolder(name) => {
                if resolvable.count() > 1
                    && (child_count == 0
                        || matches!(resolvable.last(), PathElement::ChildQualifier(_)))
                {
                    path
                } else {
                    path.join(name)
                }
            }
        }
    }
}

impl Default for ExpandStrategy {
    fn default() -> Self {
        ExpandStrategy::UniqueFolder(String::from("data"))
    }
}
