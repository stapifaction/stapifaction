use std::{borrow::Cow, iter::empty, path::PathBuf};

use erased_serde::Serialize as ErasedSerialize;
use serde::Serialize;

use crate::{PathStyle, ResolvablePath};

/// Defines how to persist a given entity.
pub trait Persist {
    /// The path where the entity will be persisted.
    fn path(&self) -> ResolvablePath;
    /// The style used to compute persistable field paths.
    fn path_style(&self) -> Option<PathStyle>;
    /// The entity that will be serialized.
    fn as_serializable<'e>(&'e self) -> Option<Box<dyn ErasedSerialize + 'e>>;
    /// The entity children.
    fn children<'e>(&'e self) -> Box<dyn Iterator<Item = (PathBuf, Cow<'e, Child<'e>>)> + 'e>;
}

/// A child.
#[derive(Clone)]
pub enum Child<'a> {
    /// A entity child.
    Entity(&'a dyn Persist),
    /// A children collections.
    Collection(Box<[Child<'a>]>),
}

impl<'a> Child<'a> {
    /// Creates a new entity.
    pub fn entity<P: Persist>(entity: &'a P) -> Self {
        Self::Entity(entity)
    }

    /// Creates a new collection.
    pub fn collection<I, P>(collection: I) -> Self
    where
        I: Iterator<Item = &'a P> + 'a,
        P: Persist + 'a,
    {
        Self::Collection(collection.map(Child::entity).collect())
    }
}

impl<'a> Persist for Child<'a> {
    fn path(&self) -> ResolvablePath {
        match self {
            Child::Entity(entity) => entity.path(),
            Child::Collection(_) => ResolvablePath::default(),
        }
    }

    fn path_style(&self) -> Option<PathStyle> {
        match self {
            Child::Entity(entity) => entity.path_style(),
            Child::Collection(_) => None,
        }
    }

    fn as_serializable<'e>(&'e self) -> Option<Box<dyn ErasedSerialize + 'e>> {
        match self {
            Child::Entity(entity) => entity.as_serializable(),
            Child::Collection(_) => None,
        }
    }

    fn children<'e>(&'e self) -> Box<dyn Iterator<Item = (PathBuf, Cow<'e, Child<'e>>)> + 'e> {
        match self {
            Child::Entity(entity) => entity.children(),
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

impl<T> Persist for Vec<T>
where
    T: Serialize,
{
    fn path(&self) -> ResolvablePath {
        ResolvablePath::default()
    }

    fn path_style(&self) -> Option<PathStyle> {
        None
    }

    fn as_serializable<'e>(&'e self) -> Option<Box<dyn ErasedSerialize + 'e>> {
        Some(Box::new(self))
    }

    fn children<'e>(&'e self) -> Box<dyn Iterator<Item = (PathBuf, Cow<'e, Child<'e>>)> + 'e> {
        Box::new(empty())
    }
}
