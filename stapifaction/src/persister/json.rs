use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use erased_serde::Serialize;
use eyre::{Context, Result};

use crate::{PathResolveStrategy, Persistable};

use super::Persister;

pub struct JsonPersister;

impl Persister for JsonPersister {
    fn resolve_path(
        &self,
        parent_path: &Path,
        entity_name: Option<PathBuf>,
        strategy: PathResolveStrategy,
    ) -> PathBuf {
        let mut path = parent_path.join(entity_name.unwrap_or_default());

        if matches!(strategy, PathResolveStrategy::SubsetsInSeparateFolders) {
            path.push("index.json");
        } else {
            path.set_extension("json");
        }

        path
    }

    fn write<'a>(&self, path: &Path, serializable: Box<dyn Serialize + 'a>) -> Result<()> {
        if let Some(parent_path) = path.parent() {
            fs::create_dir_all(parent_path)?;
        }

        let file = File::create(&path)
            .wrap_err_with(|| format!("Failed to create file '{}'", path.display()))?;

        serde_json::to_writer(file, &serializable)
            .wrap_err_with(|| format!("Failed serialize element '{:?}'", path))?;

        Ok(())
    }
}

pub trait ToJson: Persistable + Sized {
    fn to_json<P: AsRef<Path>>(&self, base_path: P) -> Result<()> {
        let persister = JsonPersister;

        persister.persist(base_path.as_ref(), self)?;

        Ok(())
    }
}

impl<T: Persistable> ToJson for T {}

pub trait ToJsonIterable: IntoIterator + Sized
where
    <Self as IntoIterator>::Item: ToJson,
{
    fn to_json<P: AsRef<Path>>(self, base_path: P) -> Result<()> {
        self.into_iter()
            .try_for_each(|p| p.to_json(base_path.as_ref()))?;

        Ok(())
    }
}

impl<I: ToJson> ToJsonIterable for Vec<I> {}
