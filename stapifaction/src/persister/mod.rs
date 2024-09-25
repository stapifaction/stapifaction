#[cfg(feature = "json")]
/// Serialize to json.
pub mod json;

use std::{fs, path::Path};

use erased_serde::Serialize as ErasedSerialize;
use eyre::{Context, Result};

use crate::{ExpandStrategy, PathElement, Persist, ResolvablePath};

/// Persister handle how entity are actually persisted on disk.
pub trait Persister {
    /// Serialize an entity.
    fn serialize<'a>(&self, path: &Path, serializable: Box<dyn ErasedSerialize + 'a>)
        -> Result<()>;

    /// Gets the file extension.
    fn extension(&self) -> String;

    /// Persists a [`Persist`] and its children.
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

            if let Some(parent_path) = resolved_path.parent() {
                fs::create_dir_all(parent_path)?;
            }

            let mut path = resolved_path.to_path_buf();

            path.set_extension(self.extension());

            self.serialize(&path, serializable)
                .wrap_err_with(|| format!("Failed serialize element '{:?}'", path))?;
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
