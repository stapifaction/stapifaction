use std::path::Path;

use erased_serde::Serialize;

pub trait Persister {
    fn persist(&self, path: &Path, serializable: &(impl Serialize + ?Sized));
}
