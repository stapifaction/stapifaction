use std::{ffi::OsString, path::PathBuf};

use erased_serde::Serialize;

use crate::Persister;

pub trait Persistable {
    fn serializables(&self) -> Vec<(PathBuf, Box<dyn Serialize>)>;
}

pub trait PersistableEntity: Serialize {
    fn id(&self) -> OsString;
    fn path(&self) -> PathBuf;
}

impl dyn PersistableEntity {
    pub fn persist<P: Persister>(&self, persister: &P) {
        persister.persist(self.path().join(self.id()), self)
    }
}

pub trait CompoundPersistable {
    fn id(&self) -> OsString;
    fn path(&self) -> PathBuf;
    fn subsets(&self) -> Vec<Box<dyn PersistableSubset>>;
}

impl dyn CompoundPersistable {
    pub fn persist<P: Persister>(&self, persister: &P) {
        for subset in self.subsets() {
            let path = self.path().join(self.id());

            let path = if let Some(sub_path) = subset.path() {
                path.join(sub_path)
            } else {
                path
            };

            persister.persist(path, subset.as_ref())
        }
    }
}

pub trait PersistableSubset: Serialize {
    fn path(&self) -> Option<PathBuf>;
}
