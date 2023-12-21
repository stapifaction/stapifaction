use std::path::{Path, PathBuf};

use crate::PathElement;

#[derive(Debug, Clone, Default)]
pub struct ResolvablePath {
    elements: Vec<PathElement>,
}

impl ResolvablePath {
    pub fn append(mut self, element: PathElement) -> Self {
        self.elements.push(element);
        self
    }

    pub fn append_all(mut self, path: ResolvablePath) -> Self {
        path.elements
            .into_iter()
            .for_each(|e| self.elements.push(e));

        self
    }

    pub fn merge(&self) -> Self {
        ResolvablePath::default().append(PathElement::Path(self.into()))
    }

    pub fn last(&self) -> &PathElement {
        self.elements.last().unwrap()
    }

    pub fn has_id(&self) -> bool {
        matches!(self.last(), PathElement::Id(_))
    }
}

impl From<&ResolvablePath> for PathBuf {
    fn from(value: &ResolvablePath) -> Self {
        let mut path = PathBuf::default();

        value
            .elements
            .iter()
            .for_each(|e| path.push(PathBuf::from(e.clone())));

        path
    }
}

impl From<&str> for ResolvablePath {
    fn from(value: &str) -> Self {
        ResolvablePath::default().append(PathElement::Path(value.into()))
    }
}

impl From<&Path> for ResolvablePath {
    fn from(value: &Path) -> Self {
        ResolvablePath::default().append(PathElement::Path(value.to_path_buf()))
    }
}
