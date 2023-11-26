use std::{collections::HashMap, path::PathBuf};

use crate::Persister;
use erased_serde::Serialize as ErasedSerialize;

pub trait Persistable {
    fn subsets<'a>(&'a self) -> HashMap<PathBuf, Box<dyn ErasedSerialize + 'a>>;

    fn persist<P: Persister>(&self, persister: &P) {
        for (path, subset) in self.subsets() {
            persister.persist(&path, &subset)
        }
    }
}
