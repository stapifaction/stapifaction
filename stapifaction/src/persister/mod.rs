#[cfg(feature = "json")]
pub mod json;

use std::{fs, path::Path};

use erased_serde::Serialize;
use eyre::Result;

use crate::Persistable;

pub trait Persister {
    fn write<'a>(&self, path: &Path, serializable: Box<dyn Serialize + 'a>) -> Result<()>;

    fn persist<P: AsRef<Path>, T: Persistable>(&self, base_path: P, persistable: &T) -> Result<()> {
        let entity_path = base_path
            .as_ref()
            .join(persistable.path().unwrap_or_default());

        fs::create_dir_all(&entity_path)?;

        if let Some(serializable_entity) = persistable.serializable_entity() {
            self.write(&entity_path, serializable_entity)?;
        }

        for (path, child) in persistable.children() {
            let child_path = entity_path.join(path.unwrap_or_default());

            self.persist(&child_path, child.as_ref())?;
        }

        Ok(())
    }
}
