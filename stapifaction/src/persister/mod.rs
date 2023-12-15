#[cfg(feature = "json")]
pub mod json;

use std::path::{Path, PathBuf};

use erased_serde::Serialize;
use eyre::Result;

use crate::Persistable;

pub trait Persister {
    fn write<'a>(
        &self,
        parent_path: &Path,
        entity_name: Option<PathBuf>,
        serializable: Box<dyn Serialize + 'a>,
    ) -> Result<()>;

    fn persist<P: AsRef<Path>, T: Persistable>(&self, base_path: P, persistable: &T) -> Result<()> {
        if let Some(serializable_entity) = persistable.serializable_entity() {
            self.write(base_path.as_ref(), persistable.path(), serializable_entity)?;
        }

        for (path, child) in persistable.children() {
            let child_path = base_path
                .as_ref()
                .join(persistable.path().unwrap_or_default())
                .join(path.unwrap_or_default());

            self.persist(&child_path, child.as_ref())?;
        }

        Ok(())
    }
}
