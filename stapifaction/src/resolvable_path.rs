use std::path::{Path, PathBuf};

use crate::PathElement;

/// A resolvable path.
#[derive(Debug, Clone, Default)]
pub struct ResolvablePath {
    elements: Vec<PathElement>,
}

impl ResolvablePath {
    /// Appends a [`PathElement`].
    pub fn append(mut self, element: PathElement) -> Self {
        self.elements.push(element);
        self
    }

    /// Appends all given [`ResolvablePath`] elements.
    pub fn append_all(mut self, path: ResolvablePath) -> Self {
        path.elements
            .into_iter()
            .for_each(|e| self.elements.push(e));

        self
    }

    /// Merges all path elements into a new [`ResolvablePath`].
    pub fn merge(&self) -> Self {
        ResolvablePath::default().append(PathElement::Path(self.into()))
    }

    /// Returns the last element in the path.
    pub fn last(&self) -> &PathElement {
        self.elements.last().unwrap()
    }

    /// Returns `true` is the last element is an id.
    pub fn has_id(&self) -> bool {
        matches!(self.last(), PathElement::Id(_))
    }

    /// Returns the number of elements in the path.
    pub fn count(&self) -> usize {
        self.elements.len()
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
