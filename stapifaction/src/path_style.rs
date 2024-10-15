use std::path::PathBuf;

use crate::{PathElement, ResolvablePath};

/// A path style defines how files are persisted.
///
/// For instance, using and the [`JsonPersister`], paths defined with
/// `PathStyle::SeparateFolders(_)` will be computed as follow:
/// * `.\*default_file_name*.json`
/// * `.\*field_name*\*default_file_name*.json`
///
/// With `PathStyle::UniqueFolder(_)`, paths will have the following form:
/// * `.\*default_file_name*.json`
/// * `.\*field_name*.json`
///
/// [`JsonPersister`]: stapifaction::JsonPersister
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PathStyle {
    /// Persists each child in a different folder.
    SeparateFolders(String),
    /// Persists each child in the same folder.
    UniqueFolder(String),
}

impl PathStyle {
    /// Compute the paths as folders.
    ///
    /// For instance, with `file_name` set to "index", and using
    /// the [`JsonPersister`], a given struct will be persisted to
    /// `.\index.json` and a given persistable field `address` will be
    /// persisted to `.\address\index.json`.
    ///
    /// [`JsonPersister`]: stapifaction::JsonPersister
    pub fn as_folders(file_name: &str) -> Self {
        Self::SeparateFolders(file_name.to_owned())
    }

    /// Compute the paths as files.
    ///
    /// For instance, with `file_name` set to "order", and using
    /// the [`JsonPersister`], a given struct will be persisted to
    /// `.\order.json` and a given persistable field `factory` will be
    /// persisted to `.\factory.json`.
    ///
    /// [`JsonPersister`]: stapifaction::JsonPersister
    pub fn as_files(file_name: &str) -> Self {
        Self::UniqueFolder(file_name.to_owned())
    }

    /// Resolves a [`ResolvablePath`] to a [`PathBuf`].
    pub fn resolve_path(&self, resolvable: &ResolvablePath, child_count: usize) -> PathBuf {
        let path = PathBuf::from(resolvable);
        match self {
            PathStyle::SeparateFolders(name) => path.join(name),
            PathStyle::UniqueFolder(name) => {
                if resolvable.count() > 1
                    && (child_count == 0
                        || matches!(resolvable.last(), PathElement::ChildQualifier(_)))
                {
                    path
                } else {
                    path.join(name)
                }
            }
        }
    }
}

impl Default for PathStyle {
    fn default() -> Self {
        PathStyle::UniqueFolder(String::from("data"))
    }
}
