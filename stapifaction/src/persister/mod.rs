#[cfg(feature = "json")]
pub mod json;

use std::path::{Path, PathBuf};

use erased_serde::Serialize;
use eyre::Result;

use crate::{ExpandStrategy, Persistable};

pub trait Persister {
    fn resolve_path(
        &self,
        parent_path: &Path,
        entity_name: Option<PathBuf>,
        strategy: ExpandStrategy,
    ) -> PathBuf;

    fn write<'a>(&self, path: &Path, serializable: Box<dyn Serialize + 'a>) -> Result<()>;

    fn persist<P: AsRef<Path>, T: Persistable>(&self, base_path: P, persistable: &T) -> Result<()> {
        if let Some(serializable_entity) = persistable.serializable_entity() {
            let path = self.resolve_path(
                base_path.as_ref(),
                persistable.path(),
                persistable.expand_strategy(),
            );
            self.write(&path, serializable_entity)?;
        }

        for (path, child) in persistable.children() {
            let child_path = base_path
                .as_ref()
                .join(persistable.path().unwrap_or_default())
                .join(path);

            self.persist(&child_path, child.as_ref())?;
        }

        Ok(())
    }
}
