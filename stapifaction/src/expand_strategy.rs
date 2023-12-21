use std::path::PathBuf;

use crate::{PathElement, ResolvablePath};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ExpandStrategy {
    SubsetsInSeparateFolders(String),
    SubsetsGroupedInUniqueFolder(String),
    IdAsFileName,
}

impl ExpandStrategy {
    pub fn resolve_path(&self, resolvable: &ResolvablePath) -> PathBuf {
        let path = PathBuf::from(resolvable);
        match self {
            ExpandStrategy::SubsetsInSeparateFolders(name) => path.join(name),
            ExpandStrategy::SubsetsGroupedInUniqueFolder(name) => {
                if matches!(resolvable.last(), PathElement::ChildQualifier(_)) {
                    path
                } else {
                    path.join(name)
                }
            }
            ExpandStrategy::IdAsFileName => path,
        }
    }
}

impl Default for ExpandStrategy {
    fn default() -> Self {
        ExpandStrategy::SubsetsGroupedInUniqueFolder(String::from("data"))
    }
}
