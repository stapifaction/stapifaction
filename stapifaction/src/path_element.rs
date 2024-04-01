use std::path::PathBuf;

/// A path element.
#[derive(Debug, Clone)]
pub enum PathElement {
    /// A regular path.
    Path(PathBuf),
    /// A path corresponding to an id.
    Id(PathBuf),
    /// A path qualifying a child.
    ChildQualifier(PathBuf),
}

impl From<PathElement> for PathBuf {
    fn from(value: PathElement) -> Self {
        match value {
            PathElement::Path(p) => p,
            PathElement::Id(p) => p,
            PathElement::ChildQualifier(p) => p,
        }
    }
}
