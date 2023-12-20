#[cfg(feature = "json")]
pub mod json;

use std::path::Path;

use erased_serde::Serialize;
use eyre::Result;

use crate::{ExpandStrategy, Persistable};

pub trait Persister {
    fn write<'a>(&self, path: &Path, serializable: Box<dyn Serialize + 'a>) -> Result<()>;

    fn persist<P: AsRef<Path>, T: Persistable>(
        &self,
        base_path: P,
        persistable: &T,
        expand_strategy: Option<ExpandStrategy>,
    ) -> Result<()> {
        let expand_strategy = persistable.expand_strategy().or(expand_strategy.clone());

        if let Some(serializable_entity) = persistable.serializable_entity() {
            let resolved_path = expand_strategy
                .clone()
                .unwrap_or_default()
                .resolve_path(&persistable.path());

            let path = base_path.as_ref().join(resolved_path);

            self.write(&path, serializable_entity)?;
        }

        for (child_path, child) in persistable.children() {
            let child_path = base_path
                .as_ref()
                .join(persistable.path().path_and_id())
                .join(child_path);

            self.persist(
                &child_path,
                child.as_ref(),
                child.expand_strategy().or(expand_strategy.clone()),
            )?;
        }

        Ok(())
    }
}
