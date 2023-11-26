use std::{collections::HashMap, ffi::OsString, path::PathBuf};

use erased_serde::Serialize as ErasedSerialize;
use serde::Serialize;

use crate::Persister;

pub struct Persistable {
    subsets: HashMap<PathBuf, Box<dyn ErasedSerialize>>,
}

impl Persistable {
    pub fn from_entity(entity: impl PersistableEntity + 'static) -> Self {
        Self {
            subsets: HashMap::from([(
                entity.path().join(entity.id()),
                Box::new(entity) as Box<dyn ErasedSerialize>,
            )]),
        }
    }

    pub fn from_compound(compound: impl CompoundPersistable + 'static) -> Self {
        Self {
            subsets: compound.subsets(),
        }
    }

    pub fn persist<P: Persister>(&self, persister: &P) {
        for (path, subset) in &self.subsets {
            persister.persist(path, &subset)
        }
    }
}

pub trait PersistableEntity: Serialize {
    fn id(&self) -> OsString;
    fn path(&self) -> PathBuf;
}

pub trait CompoundPersistable {
    fn id(&self) -> OsString;
    fn path(&self) -> PathBuf;
    fn subsets(&self) -> HashMap<PathBuf, Box<dyn ErasedSerialize>>;
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
