use std::{collections::HashMap, path::PathBuf};

use erased_serde::Serialize as ErasedSerialize;

pub trait Persistable {
    fn path() -> Option<PathBuf>;

    fn subsets<'a>(&'a self) -> HashMap<Option<PathBuf>, Box<dyn ErasedSerialize + 'a>>;
}
