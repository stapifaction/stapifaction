use std::{fs::File, path::Path};

use erased_serde::Serialize;
use eyre::{Context, Result};

use crate::Persist;

use super::Persister;

/// A persister that saves entities as JSON files.
pub struct JsonPersister;

impl Persister for JsonPersister {
    fn serialize<'a>(&self, path: &Path, serializable: Box<dyn Serialize + 'a>) -> Result<()> {
        let file = File::create(&path)
            .wrap_err_with(|| format!("Failed to create file '{}'", path.display()))?;

        serde_json::to_writer(file, &serializable)
            .wrap_err_with(|| format!("Failed serialize element '{:?}'", path))?;

        Ok(())
    }

    fn extension(&self) -> String {
        String::from("json")
    }
}

/// Extension trait allowing to persist as JSON files.
pub trait ToJson: Persist + Sized {
    /// Persists to JSON files at the given path.
    fn to_json<P: AsRef<Path>>(&self, base_path: P) -> Result<()> {
        let persister = JsonPersister;

        persister.persist(base_path.as_ref(), self, self.path_style())?;

        Ok(())
    }
}

impl<T: Persist> ToJson for T {}

/// Extension trait allowing to persist a collection as JSON files.
pub trait ToJsonIterable: IntoIterator + Sized
where
    <Self as IntoIterator>::Item: ToJson,
{
    /// Persists the collection to JSON files at the given path.
    fn to_json<P: AsRef<Path>>(self, base_path: P) -> Result<()> {
        self.into_iter()
            .try_for_each(|p| p.to_json(base_path.as_ref()))?;

        Ok(())
    }
}

impl<I: ToJson> ToJsonIterable for Vec<I> {}
