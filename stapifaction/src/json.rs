use std::{
    fs::{self, File},
    path::Path,
};

use eyre::{Context, Result};

use crate::Persistable;

pub trait ToJson: Persistable {
    fn to_json<P: AsRef<Path>>(&self, base_path: P) -> Result<()> {
        let entity_path = base_path.as_ref().join(Self::path());
        fs::create_dir_all(&entity_path)?;

        for (path, subset) in self.subsets() {
            let full_path = entity_path.join(&path);
            let file = File::create(&full_path)
                .wrap_err_with(|| format!("Failed to create file '{}'", full_path.display()))?;
            serde_json::to_writer(file, &subset)
                .wrap_err_with(|| format!("Failed serialize subset '{}'", path.display()))?;
        }

        Ok(())
    }
}

impl<T: Persistable> ToJson for T {}
