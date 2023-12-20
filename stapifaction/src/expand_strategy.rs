use std::path::PathBuf;

use crate::ResolvablePath;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ExpandStrategy {
    SubsetsInSeparateFolders(String),
    SubsetsGroupedInUniqueFolder(String),
    IdAsFileName,
}

impl ExpandStrategy {
    pub fn resolve_path(&self, resolvable: &ResolvablePath) -> PathBuf {
        let mut path = resolvable.path();

        match self {
            ExpandStrategy::SubsetsInSeparateFolders(name) => {
                path.push(resolvable.id());
                path.push(name);
            }
            ExpandStrategy::SubsetsGroupedInUniqueFolder(name) => {
                path.push(
                    resolvable
                        .id
                        .clone()
                        .map(|e| e.join(name))
                        .unwrap_or_default(),
                );
            }
            ExpandStrategy::IdAsFileName => {
                path.push(resolvable.id.clone().unwrap());
            }
        };

        path
    }
}

impl Default for ExpandStrategy {
    fn default() -> Self {
        ExpandStrategy::SubsetsInSeparateFolders(String::from("index"))
    }
}
