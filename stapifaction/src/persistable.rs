use std::{ffi::OsStr, path::PathBuf};

use erased_serde::Serialize;

pub trait Persistable {
    fn serializables(&self) -> Vec<(PathBuf, Box<dyn Serialize>)>;
}

pub trait PersistableEntity: Serialize {
    fn id(&self) -> OsStr;
    fn path(&self) -> PathBuf;
}

impl dyn PersistableEntity {
    pub fn persist(&self) {
        todo!()
    }
}

pub trait CompoundPersistable {
    fn id(&self) -> OsStr;
    fn subsets(&self) -> [Box<dyn PersistableSubset>];
}

impl dyn CompoundPersistable {
    pub fn persist(&self) {
        todo!()
    }
}

pub trait PersistableSubset: Serialize {
    fn path(&self) -> Option<PathBuf>;
}
