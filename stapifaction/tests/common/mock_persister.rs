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
        let not_expected = actual_paths
            .difference(&expected_paths)
            .map(|p| p.to_str().unwrap())
            .collect::<Vec<_>>();
        let not_produced = expected_paths
            .difference(&actual_paths)
            .map(|p| p.to_str().unwrap())
            .collect::<Vec<_>>();

        assert!(
            not_expected.is_empty(),
            "These paths aren't expected: {}",
            not_expected.join(", ")
        );
        assert!(
            not_produced.is_empty(),
            "These expected paths weren't produced: {}",
            not_produced.join(", ")
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
