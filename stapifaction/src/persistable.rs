use std::{borrow::Cow, path::PathBuf};

use erased_serde::Serialize as ErasedSerialize;

use crate::{ExpandStrategy, ResolvablePath};

/// Persistable defines how to persist a given entity.
pub trait Persistable {
    /// The path where the entity will be persisted.
    fn path(&self) -> ResolvablePath;
    /// The strategy used to expand childrens.
    fn expand_strategy(&self) -> Option<ExpandStrategy>;
    /// The entity that will be serialized.
    fn serializable_entity<'e>(&'e self) -> Option<Box<dyn ErasedSerialize + 'e>>;
    /// The entity children.
    fn children<'e>(&'e self) -> Box<dyn Iterator<Item = (PathBuf, Cow<'e, Child<'e>>)> + 'e>;
}

/// A child.
#[derive(Clone)]
pub enum Child<'a> {
    /// A subset child.
    Subset(&'a dyn Persistable),
    /// A children collections.
    Collection(Box<[Child<'a>]>),
}

impl<'a> Child<'a> {
    /// Creates a new subset.
    pub fn subset<P: Persistable>(subset: &'a P) -> Self {
        Self::Subset(subset)
    }

    /// Creates a new collection.
    pub fn collection<I, P>(collection: I) -> Self
    where
        I: Iterator<Item = &'a P> + 'a,
        P: Persistable + 'a,
    {
        Self::Collection(collection.map(Child::subset).collect())
    }
}

impl<'a> Persistable for Child<'a> {
    fn path(&self) -> ResolvablePath {
        match self {
            Child::Subset(subset) => subset.path(),
            Child::Collection(_) => ResolvablePath::default(),
        }
    }

    fn expand_strategy(&self) -> Option<ExpandStrategy> {
        match self {
            Child::Subset(subset) => subset.expand_strategy(),
            Child::Collection(_) => None,
        }
    }

    fn serializable_entity<'e>(&'e self) -> Option<Box<dyn ErasedSerialize + 'e>> {
        match self {
            Child::Subset(subset) => subset.serializable_entity(),
            Child::Collection(_) => None,
        }
    }

    fn children<'e>(&'e self) -> Box<dyn Iterator<Item = (PathBuf, Cow<'e, Child<'e>>)> + 'e> {
        match self {
            Child::Subset(subset) => subset.children(),
            Child::Collection(collection) => {
                Box::new(collection.iter().enumerate().map(|(index, child)| {
                    let path = if child.path().has_id() {
                        PathBuf::default()
                    } else {
                        index.to_string().into()
                    };

                    (path, Cow::Borrowed(child))
                }))
            }
        }
    }
}
