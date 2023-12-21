use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum PathElement {
    Path(PathBuf),
    Id(PathBuf),
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
