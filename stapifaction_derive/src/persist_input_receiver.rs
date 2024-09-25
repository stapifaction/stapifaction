use std::{borrow::Cow, collections::HashSet, hash::Hash};

use darling::{util::Override, FromDeriveInput, FromField, FromMeta};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde_derive_internals::ast::{Container, Data};
use syn::{Member, Type};

pub fn expand_derive_persist(serde_container: Container) -> TokenStream {
    let Container {
        ident,
        data,
        original,
        ..
    } = serde_container;
    let PersistInputReceiver {
        path,
        expand_strategy,
    } = PersistInputReceiver::from_derive_input(original).unwrap();

    let container_ident = format_ident!("{}Container", ident);

    if let Data::Struct(_, fields) = data {
        let fields = fields
            .into_iter()
            .map(|serde_field| {
                PersistableField::from_field(serde_field.original).map(|f| (serde_field, f))
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let (main_set, others) = fields
            .into_iter()
            .filter(|(f, p)| !f.attrs.skip_serializing() || p.expand.is_some())
            .partition::<Vec<_>, _>(|(_, f)| f.expand().is_none());

        let id = main_set.iter().find(|(_, f)| f.id).map(|(f, _)| &f.member);

        let expand_strategy = expand_strategy.map(|expand_strategy| match expand_strategy {
                ExpandStrategy::SeparateFolders => {
                    quote! { Some(stapifaction::ExpandStrategy::entitiesInSeparateFolders(format!("index"))) }
                }
                ExpandStrategy::SameFolder => {
                    quote! { Some(stapifaction::ExpandStrategy::entitiesGroupedInUniqueFolder(format!("data"))) }
                }
            }).unwrap_or_else(|| 
                quote! { None }
            );

        let (entities, collections) = others
            .into_iter()
            .partition::<Vec<_>, _>(|(_, f)| matches!(*f.expand().unwrap(), Expand::Entity));

        let resolvable_path = quote! { stapifaction::ResolvablePath::default() };

        let resolvable_path = match path {
            Some(path) => {
                quote! { #resolvable_path.append(stapifaction::PathElement::Path(String::from(#path).into())) }
            }
            None => resolvable_path,
        };

        let resolvable_path = match id {
            Some(id) => {
                quote! { #resolvable_path.append(stapifaction::PathElement::Id(format!("{}",self.#id).into())) }
            }
            None => resolvable_path,
        };

        let (entity_idents_str, entities) = entities
            .iter()
            .filter_map(|(f, _)| match &f.member {
                Member::Named(ident) => Some((
                    f.attrs.name().serialize_name(),
                    (
                        build_path_buf(f.attrs.name().serialize_name()),
                        ident,
                        is_option(f.ty),
                    ),
                )),
                Member::Unnamed(_) => None,
            })
            .sorted_by(|(a, _), (b, _)| a.cmp(b))
            .multiunzip::<(Vec<_>, Vec<_>)>();

        let duplicated_entities = collect_duplicates(&entity_idents_str);

        if !duplicated_entities.is_empty() {
            panic!(
                "The following entities are duplicated: {}",
                duplicated_entities.into_iter().join(", ")
            );
        }

        let (optional_entities, entities) = entities
            .into_iter()
            .partition::<Vec<_>, _>(|(_, _, is_option)| *is_option);

        let (entity_path_buf, entity_idents) = entities
            .into_iter()
            .map(|(entity_path_buf, entity_idents, _)| (entity_path_buf, entity_idents))
            .unzip::<_, _, Vec<_>, Vec<_>>();

        let (optional_entity_path_buf, optional_entity_idents) = optional_entities
            .into_iter()
            .map(|(entity_path_buf, entity_idents, _)| (entity_path_buf, entity_idents))
            .unzip::<_, _, Vec<_>, Vec<_>>();

        let (collection_idents_str, collection_path_buf, collection_idents) = collections
            .iter()
            .filter_map(|(f, _)| match &f.member {
                Member::Named(ident) => Some((
                    f.attrs.name().serialize_name(),
                    build_path_buf(f.attrs.name().serialize_name()),
                    ident,
                )),
                Member::Unnamed(_) => None,
            })
            .sorted_by(|(a, _, _), (b, _, _)| a.cmp(b))
            .multiunzip::<(Vec<_>, Vec<_>, Vec<_>)>();

        let duplicated_collections = collect_duplicates(&collection_idents_str);

        if !duplicated_collections.is_empty() {
            panic!(
                "The following collections are duplicated: {}",
                duplicated_collections.into_iter().join(", ")
            );
        }

        quote! {
            struct #container_ident<'a> {
                entity: &'a #ident,
            }

            impl stapifaction::Persist for #ident {
                fn path(&self) -> stapifaction::ResolvablePath {
                    #resolvable_path
                }

                fn expand_strategy(&self) -> Option<stapifaction::ExpandStrategy> {
                    #expand_strategy
                }

                fn as_serializable<'e>(&'e self) -> Option<Box<dyn stapifaction::ErasedSerialize + 'e>> {
                    Some(Box::new(self) as Box<dyn stapifaction::ErasedSerialize>)
                }

                fn children<'e>(&'e self) -> Box<dyn Iterator<Item = (std::path::PathBuf, std::borrow::Cow<'e, stapifaction::Child<'e>>)> + 'e>
                {
                    let mut map = std::collections::HashMap::new();

                    #(
                        map.insert(
                            #entity_path_buf,
                            std::borrow::Cow::Owned(stapifaction::Child::entity(&self.#entity_idents))
                        );
                    )*

                    #(
                        if let Some(entity) = &self.#optional_entity_idents {
                            map.insert(
                                #optional_entity_path_buf,
                                std::borrow::Cow::Owned(stapifaction::Child::entity(entity))
                            );
                        }
                    )*

                    #(
                        map.insert(
                            #collection_path_buf,
                            std::borrow::Cow::Owned(stapifaction::Child::collection(self.#collection_idents.iter()))
                        );
                    )*

                    Box::new(map.into_iter())
                }
            }
        }
    } else {
        unimplemented!("Enums aren't supported");
    }
}

fn collect_duplicates<T>(iter: T) -> Vec<T::Item>
where
    T: IntoIterator,
    T::Item: Eq + Hash + Clone,
{
    let mut uniq = HashSet::new();
    iter.into_iter()
        .filter(|x| !uniq.insert(x.clone()))
        .dedup()
        .collect()
}

fn build_path_buf(path: &str) -> TokenStream {
    quote! { std::path::PathBuf::from(#path) }
}

fn is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .map(|s| s.ident == "Option")
            .unwrap_or_default(),
        _ => false,
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(persist), supports(struct_any))]
pub struct PersistInputReceiver {
    pub path: Option<String>,
    pub expand_strategy: Option<ExpandStrategy>,
}

#[derive(Debug, FromField)]
#[darling(attributes(persist))]
pub struct PersistableField {
    #[darling(default)]
    pub id: bool,
    pub expand: Option<Override<Expand>>,
}

impl PersistableField {
    pub fn expand(&self) -> Option<Cow<'_, Expand>> {
        match &self.expand {
            Some(expand) => match expand {
                Override::Explicit(value) => Some(Cow::Borrowed(value)),
                Override::Inherit => Some(Cow::Owned(Expand::Entity)),
            },
            None => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, FromMeta)]
#[darling(default)]
pub enum Expand {
    #[default]
    Entity,
    All,
}

#[derive(Debug, Clone, Copy, FromMeta)]
#[darling(rename_all = "kebab-case")]
pub enum ExpandStrategy {
    SeparateFolders,
    SameFolder,
}

#[cfg(test)]
mod tests {
    use darling::FromDeriveInput;
    use syn::parse_quote;

    use crate::persist_input_receiver::PersistInputReceiver;

    #[test]
    fn test_persistable_entity() {
        let di = parse_quote! {
            #[derive(Persist)]
            #[persist(path = "users")]
            pub struct User {
                #[persist(id)]
                user_name: String,
                first_name: String,
                last_name: String,
            }
        };

        let receiver = PersistInputReceiver::from_derive_input(&di).unwrap();

        assert_eq!(receiver.path.unwrap(), "users");
    }
}
