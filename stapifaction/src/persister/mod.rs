use std::path::PathBuf;

use erased_serde::Serialize;

pub trait Persister {
    fn persist(&self, path: PathBuf, serializable: &(impl Serialize + ?Sized));
}
