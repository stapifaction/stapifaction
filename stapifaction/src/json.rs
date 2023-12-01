use std::{
    fs::{self, File},
    path::Path,
};

use eyre::{Context, Result};

use crate::Persistable;

pub trait ToJson: Persistable {
    fn to_json<P: AsRef<Path>>(&self, base_path: P) -> Result<()> {
        let entity_path = Self::path()
            .map(|p| base_path.as_ref().join(p))
            .unwrap_or(base_path.as_ref().to_path_buf());

        for (path, subset) in self.subsets() {
            let full_path = match &path {
                Some(path) => entity_path.join(path),
                None => entity_path.clone(),
            };

            let full_path = full_path.join("index.json");

            if let Some(parent) = full_path.parent() {
                println!("{parent:?}");
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }

            let file = File::create(&full_path)
                .wrap_err_with(|| format!("Failed to create file '{}'", full_path.display()))?;

            serde_json::to_writer(file, &subset)
                .wrap_err_with(|| format!("Failed serialize subset '{:?}'", path))?;
        }

        Ok(())
    }
}

impl<T: Persistable> ToJson for T {}
