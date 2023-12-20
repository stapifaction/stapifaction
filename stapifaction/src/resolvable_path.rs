use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct ResolvablePath {
    pub path: Option<PathBuf>,
    pub id: Option<PathBuf>,
}

impl ResolvablePath {
    pub fn new(path: Option<PathBuf>, id: Option<PathBuf>) -> Self {
        Self { path, id }
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone().unwrap_or_default()
    }

    pub fn id(&self) -> PathBuf {
        self.id.clone().unwrap_or_default()
    }

    pub fn or_with_id(mut self, id: PathBuf) -> Self {
        let _ = self.id.get_or_insert(id);
        self
    }

    pub fn path_and_id(&self) -> PathBuf {
        self.path
            .clone()
            .unwrap_or_default()
            .join(self.id.clone().unwrap_or_default())
    }
}
