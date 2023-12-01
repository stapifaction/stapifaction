use std::{collections::HashMap, path::PathBuf};

use erased_serde::Serialize as ErasedSerialize;

pub trait Persistable {
    fn serializable_entity<'e>(&'e self) -> (Option<PathBuf>, Box<dyn ErasedSerialize + 'e>);
    fn subsets(&self) -> HashMap<Option<PathBuf>, Box<Subset>>;
}

impl<T: Persistable> Persistable for &T {
    fn serializable_entity<'e>(&'e self) -> (Option<PathBuf>, Box<dyn ErasedSerialize + 'e>) {
        (*self).serializable_entity()
    }

    fn subsets(&self) -> HashMap<Option<PathBuf>, Box<Subset>> {
        (*self).subsets()
    }
}

pub struct Subset<'a> {
    subset: Box<dyn Persistable + 'a>,
}

impl<'a> Subset<'a> {
    pub fn new<T: Persistable + 'a>(subset: T) -> Self {
        Self {
            subset: Box::new(subset),
        }
    }
}

impl<'a> Persistable for Subset<'a> {
    fn serializable_entity<'e>(&'e self) -> (Option<PathBuf>, Box<dyn ErasedSerialize + 'e>) {
        self.subset.serializable_entity()
    }

    fn subsets(&self) -> HashMap<Option<PathBuf>, Box<Subset>> {
        self.subset.subsets()
    }
}
