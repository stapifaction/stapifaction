use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::RwLock,
};

use erased_serde::Serialize;
use eyre::Result;
use stapifaction::Persister;

pub struct MockPersister {
    paths: RwLock<HashSet<PathBuf>>,
}

impl MockPersister {
    pub fn new() -> Self {
        Self {
            paths: RwLock::new(HashSet::new()),
        }
    }

    pub fn assert<S: Into<HashSet<PathBuf>>>(&self, expected_paths: S) {
        let expected_paths = expected_paths.into();
        let actual_paths = self.paths.read().unwrap();

        assert_eq!(
            HashSet::from([]),
            actual_paths
                .difference(&expected_paths)
                .collect::<HashSet<&PathBuf>>(),
            "These paths aren't expected"
        );
        assert_eq!(
            HashSet::from([]),
            expected_paths
                .difference(&actual_paths)
                .collect::<HashSet<&PathBuf>>(),
            "These expected paths weren't produced"
        );
    }
}

impl Persister for MockPersister {
    fn serialize<'x>(&self, path: &Path, _serializable: Box<dyn Serialize + 'x>) -> Result<()> {
        if !self.paths.write().unwrap().insert(path.to_owned()) {
            panic!("The path '{}' has been added twice", path.display())
        }

        Ok(())
    }

    fn extension(&self) -> String {
        String::from("json")
    }
}
