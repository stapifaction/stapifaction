use std::{
    fs::{self, File},
    path::Path,
};

use erased_serde::Serialize;
use eyre::{Context, Result};

use crate::Persist;

use super::Persister;

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

pub trait ToJson: Persist + Sized {
    fn to_json<P: AsRef<Path>>(&self, base_path: P) -> Result<()> {
        let persister = JsonPersister;

        persister.persist(base_path.as_ref(), self, None)?;

        Ok(())
    }
}

impl<T: Persist> ToJson for T {}

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
