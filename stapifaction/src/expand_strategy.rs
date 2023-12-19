use std::path::{Path, PathBuf};

pub enum ExpandStrategy {
    SubsetsInSeparateFolders(String),
    SubsetsGroupedInUniqueFolder(String),
    IdAsFileName,
}

impl ExpandStrategy {
    pub fn resolve_path(&self, parent_path: &Path, entity_name: Option<PathBuf>) -> PathBuf {
        let mut path = parent_path.to_path_buf(); //.join();

        match self {
            ExpandStrategy::SubsetsInSeparateFolders(name) => {
                path.push(entity_name.unwrap_or_default());
                path.push(name);
            }
            ExpandStrategy::SubsetsGroupedInUniqueFolder(name) => {
                path.push(entity_name.map(|e| e.join(name)).unwrap_or_default());
            }
            ExpandStrategy::IdAsFileName => {}
        }

        path
    }
}
