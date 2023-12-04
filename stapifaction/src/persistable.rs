use std::{borrow::Cow, fs, path::PathBuf};

use crate::persister::Persister;
use erased_serde::Serialize as ErasedSerialize;
use eyre::Result;

pub trait Persistable {
    fn path(&self) -> Option<PathBuf>;
    fn serializable_entity<'e>(&'e self) -> Option<Box<dyn ErasedSerialize + 'e>>;
    fn children<'e>(
        &'e self,
    ) -> Box<dyn Iterator<Item = (Option<PathBuf>, Cow<'e, Child<'e>>)> + 'e>;
}

#[derive(Clone)]
pub enum Child<'a> {
    Subset(&'a dyn Persistable),
    Collection(Box<[Child<'a>]>),
}

impl<'a> Child<'a> {
    pub fn subset<P: Persistable>(subset: &'a P) -> Self {
        Self::Subset(subset)
    }

    pub fn collection<I, P>(collection: I) -> Self
    where
        I: Iterator<Item = &'a P> + 'a,
        P: Persistable + 'a,
    {
        Self::Collection(collection.map(Child::subset).collect())
    }
}

impl<'a> Persistable for Child<'a> {
    fn path(&self) -> Option<PathBuf> {
        match self {
            Child::Subset(subset) => subset.path(),
            Child::Collection(_) => None,
        }
    }

    fn serializable_entity<'e>(&'e self) -> Option<Box<dyn ErasedSerialize + 'e>> {
        match self {
            Child::Subset(subset) => subset.serializable_entity(),
            Child::Collection(_) => None,
        }
    }

    fn children<'e>(
        &'e self,
    ) -> Box<dyn Iterator<Item = (Option<PathBuf>, Cow<'e, Child<'e>>)> + 'e> {
        match self {
            Child::Subset(subset) => subset.children(),
            Child::Collection(collection) => {
                Box::new(collection.iter().enumerate().map(|(index, child)| {
                    let path = match child.path() {
                        Some(path) => path,
                        None => PathBuf::from(index.to_string()),
                    };

                    (Some(path), Cow::Borrowed(child))
                }))
            }
        }
    }
}
