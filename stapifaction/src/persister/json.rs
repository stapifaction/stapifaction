use std::{fs::File, path::Path};

use erased_serde::Serialize;
use eyre::{Context, Result};

use crate::{Child, PathStyle, Persist};

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
    /// Persists to JSON files at the given path, using the path style defined
    /// as derive attribute if any.
    fn to_json<P: AsRef<Path>>(&self, base_path: P) -> Result<()> {
        let persister = JsonPersister;

        persister.persist(base_path.as_ref(), self, None)?;

        Ok(())
    }

    /// Persists to JSON files at the given path, using the given path style.
    fn to_json_with_path_style<P: AsRef<Path>, PS: Into<PathStyle>>(
        &self,
        base_path: P,
        path_style: PS,
    ) -> Result<()> {
        let persister = JsonPersister;

        persister.persist(base_path.as_ref(), self, Some(path_style.into()))?;

        Ok(())
    }
}

impl<T: Persist> ToJson for T {}

/// Extension trait allowing to persist a collection as JSON files.
pub trait IterableToJson<'a, I>: IntoIterator<Item = &'a I> + Sized + 'a
where
    I: Persist + 'a,
{
    /// Persists the collection to JSON files at the given path.
    fn items_to_json<P: AsRef<Path>>(self, base_path: P, path_style: PathStyle) -> Result<()> {
        let entities = self.into_iter();
        let collection = Child::collection(entities);
        let persister = JsonPersister;

        persister.persist(base_path.as_ref(), &collection, Some(path_style))?;

        Ok(())
    }
}

impl<'a, I, T> IterableToJson<'a, I> for &'a T
where
    &'a T: IntoIterator<Item = &'a I> + 'a,
    I: Persist + 'a,
{
}
