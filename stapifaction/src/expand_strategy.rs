use std::path::PathBuf;

use crate::{PathElement, ResolvablePath};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ExpandStrategy {
    SeparateFolders(String),
    UniqueFolder(String),
}

impl ExpandStrategy {
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
