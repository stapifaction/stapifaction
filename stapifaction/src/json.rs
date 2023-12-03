use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use erased_serde::Serialize;
use eyre::{Context, Result};

use crate::Persistable;

pub trait ToJson: Persistable {
    fn to_json<P: AsRef<Path>>(&self, base_path: P) -> Result<()> {
        let entity_path = self.path();

        let entity_path = entity_path
            .map(|p| base_path.as_ref().join(p))
            .unwrap_or(base_path.as_ref().to_path_buf());

        fs::create_dir_all(&entity_path)?;

        if let Some(serializable_entity) = self.serializable_entity() {
            serialize_file(entity_path.join("index.json"), serializable_entity)?;
        }

        for (path, child) in self.children() {
            let child_path = path
                .map(|p| entity_path.join(p))
                .unwrap_or(entity_path.clone());

            ToJson::to_json(child.as_ref(), child_path)?;
        }

        Ok(())
    }
}

impl<T: Persistable> ToJson for T {}

fn serialize_file<'a>(full_path: PathBuf, serializable: Box<dyn Serialize + 'a>) -> Result<()> {
    let file = File::create(&full_path)
        .wrap_err_with(|| format!("Failed to create file '{}'", full_path.display()))?;

    serde_json::to_writer(file, &serializable)
        .wrap_err_with(|| format!("Failed serialize subset '{:?}'", full_path))?;

    Ok(())
}
