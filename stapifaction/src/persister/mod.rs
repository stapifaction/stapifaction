#[cfg(feature = "json")]
pub mod json;

use std::path::Path;

use erased_serde::Serialize;
use eyre::Result;

use crate::{ExpandStrategy, PathElement, Persistable, ResolvablePath};

pub trait Persister {
    fn write<'a>(&self, path: &Path, serializable: Box<dyn Serialize + 'a>) -> Result<()>;

    fn persist<P: Into<ResolvablePath>, T: Persistable>(
        &self,
        base_path: P,
        persistable: &T,
        expand_strategy: Option<ExpandStrategy>,
    ) -> Result<()> {
        let expand_strategy = expand_strategy.or(persistable.expand_strategy());
        let base_path = base_path.into().append_all(persistable.path());

        if let Some(serializable_entity) = persistable.serializable_entity() {
            let resolved_path = expand_strategy
                .clone()
                .unwrap_or_default()
                .resolve_path(&base_path);

            self.write(&resolved_path, serializable_entity)?;
        }

        for (child_path, child) in persistable.children() {
            self.persist(
                base_path
                    .clone()
                    .append(PathElement::ChildQualifier(child_path)),
                child.as_ref(),
                child.expand_strategy().or(expand_strategy.clone()),
            )?;
        }

        Ok(())
    }
}
