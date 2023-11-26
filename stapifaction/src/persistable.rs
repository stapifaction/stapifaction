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

#[cfg(test)]
mod tests {
    use std::{
        ffi::OsString,
        path::{Path, PathBuf},
    };

    use serde::Serialize;

    use crate::{PersistableEntity, Persister};

    use super::Persistable;

    #[derive(Serialize)]
    struct Person;

    struct AssertPathPersister(PathBuf);

    impl PersistableEntity for Person {
        fn id(&self) -> OsString {
            "1".parse().unwrap()
        }

        fn path(&self) -> PathBuf {
            "persons".parse().unwrap()
        }
    }

    impl Persister for AssertPathPersister {
        fn persist(&self, path: &Path, _serializable: &(impl erased_serde::Serialize + ?Sized)) {
            assert_eq!(self.0, path)
        }
    }

    #[test]
    fn test_persistable_entity_path() {
        let persistable = Persistable::from_entity(Person);
        let persister = AssertPathPersister("persons/1".parse().unwrap());

        persistable.persist(&persister);
    }
}
