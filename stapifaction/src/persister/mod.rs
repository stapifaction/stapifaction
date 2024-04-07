#[cfg(feature = "json")]
pub mod json;

use std::path::Path;

use erased_serde::Serialize;
use eyre::Result;

use crate::{ExpandStrategy, PathElement, Persist, ResolvablePath};

/// Persister handle how entity are actually persisted.
pub trait Persister {
    /// Writes an entity.
    fn write<'a>(&self, path: &Path, serializable: Box<dyn Serialize + 'a>) -> Result<()>;

    /// Persists a [`Persistable`] and its children.
    fn persist<P: Into<ResolvablePath>, T: Persist>(
        &self,
        base_path: P,
        persistable: &T,
        expand_strategy: Option<ExpandStrategy>,
    ) -> Result<()> {
        let expand_strategy = expand_strategy.or(persistable.expand_strategy());
        let base_path = base_path.into().append_all(persistable.path());
        let children = persistable.children().collect::<Vec<_>>();

        if let Some(serializable) = persistable.as_serializable() {
            let resolved_path = expand_strategy
                .clone()
                .unwrap_or_default()
                .resolve_path(&base_path, children.len());

            self.write(&resolved_path, serializable)?;
        }

        for (child_path, child) in children {
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
