use std::path::PathBuf;

use crate::{PathElement, ResolvablePath};

/// A path style is used to determine the file name of the entity being
/// persisted. It's also used to determine the path of the entity's fields
/// that are persisted separately (using the `#[persist]` derive attribute).
///
/// When resolving path styles, there are 2 cases:
///
/// 1. Resolving the path for the current struct file
/// 2. Resolving the paths for all fields persisted separately
///
/// For the first case, when using `PathStyle::SameFileNameInSeparateFolders`,
/// the file name will be the
///
/// For instance, using and the [`JsonPersister`], paths defined with
/// `PathStyle::SameFileNameInSeparateFolders(*default_file_name*)` will be
/// computed as follow:
///
/// * `.\*default_file_name*.json`
/// * `.\*field_name*\*default_file_name*.json`
///
/// With `PathStyle::DifferentFileNameInSameFolder(*default_file_name*)`, paths
/// will have the following form:
///
/// * `.\*default_file_name*.json`
/// * `.\*field_name*.json`
///
/// As you can see, `PathStyle::SameFileNameInSeparateFolders` is better suited
/// to build static REST API.
/// 
/// [`JsonPersister`]: stapifaction::JsonPersister
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PathStyle {
    /// Persists each child in a different folder.
    SameFileNameInSeparateFolders(String),
    /// Persists each child in the same folder.
    DifferentFileNameInSameFolder(String),
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
        Self::SameFileNameInSeparateFolders(file_name.to_owned())
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
        Self::DifferentFileNameInSameFolder(file_name.to_owned())
    }

    /// Resolves a [`ResolvablePath`] to a [`PathBuf`].
    pub fn resolve_path(&self, resolvable: &ResolvablePath, child_count: usize) -> PathBuf {
        let path = PathBuf::from(resolvable);
        match self {
            PathStyle::SameFileNameInSeparateFolders(name) => path.join(name),
            PathStyle::DifferentFileNameInSameFolder(name) => {
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
        PathStyle::DifferentFileNameInSameFolder(String::from("data"))
    }
}

impl From<String> for PathStyle {
    fn from(value: String) -> Self {
        PathStyle::DifferentFileNameInSameFolder(value)
    }
}

impl<'a> From<&'a str> for PathStyle {
    fn from(s: &'a str) -> Self {
        PathStyle::as_files(s)
    }
}
