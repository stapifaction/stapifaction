use std::{collections::HashMap, path::PathBuf};

use erased_serde::Serialize as ErasedSerialize;

pub trait Persistable {
    fn subsets<'a>(&'a self) -> HashMap<PathBuf, Box<dyn ErasedSerialize + 'a>>;
}
