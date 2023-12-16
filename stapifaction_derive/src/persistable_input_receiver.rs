use std::{borrow::Cow, collections::HashSet, hash::Hash};

use darling::{util::Override, FromDeriveInput, FromField, FromMeta};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde_derive_internals::ast::{Container, Data};
use syn::{Ident, Member, Type};

pub fn expand_derive_persistable(serde_container: Container) -> TokenStream {
    let Container {
        ident,
        data,
        original,
        ..
    } = serde_container;
    let PersistableInputReceiver { path } =
        PersistableInputReceiver::from_derive_input(original).unwrap();

    let ident_str = ident.to_string();
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
            .filter(|(f, _)| !f.attrs.skip_serializing())
            .partition::<Vec<_>, _>(|(_, f)| f.expand().is_none());

        let (subsets, collections) = others
            .into_iter()
            .partition::<Vec<_>, _>(|(_, f)| matches!(*f.expand().unwrap(), Expand::Subset));

        let id = main_set.iter().find(|(_, f)| f.id).map(|(f, _)| &f.member);

        let path = match (path, id) {
            (Some(path), Some(id)) => quote! { Some(format!("{}/{}", #path, self.#id).into()) },
            (Some(path), None) => quote! { Some(String::from(#path).into()) },
            (None, Some(id)) => quote! { Some(format!("{}", self.#id).into()) },
            _ => quote! { None },
        };

        let fields_count = main_set.len();
        let (field_idents_str, field_idents) = main_set
            .iter()
            .filter_map(|(f, _)| match &f.member {
                Member::Named(ident) => Some((f.attrs.name().serialize_name(), ident)),
                Member::Unnamed(_) => None,
            })
            .multiunzip::<(Vec<_>, Vec<_>)>();

        let (subset_idents_str, subsets) = subsets
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

        let duplicated_subsets = collect_duplicates(&subset_idents_str);

        if !duplicated_subsets.is_empty() {
            panic!(
                "The following subsets are duplicated: {}",
                duplicated_subsets.into_iter().join(", ")
            );
        }

        let (optional_subsets, subsets) = subsets
            .into_iter()
            .partition::<Vec<_>, _>(|(_, _, is_option)| *is_option);

        let (subset_path_buf, subset_idents) = subsets
            .into_iter()
            .map(|(subset_path_buf, subset_idents, _)| (subset_path_buf, subset_idents))
            .unzip::<_, _, Vec<_>, Vec<_>>();

        let (optional_subset_path_buf, optional_subset_idents) = optional_subsets
            .into_iter()
            .map(|(subset_path_buf, subset_idents, _)| (subset_path_buf, subset_idents))
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

            impl<'a> stapifaction::serde::Serialize for #container_ident<'a> {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: stapifaction::serde::Serializer,
                {
                    let mut state = serializer.serialize_struct(#ident_str, #fields_count)?;

                    #( stapifaction::serde::SerializeStruct::serialize_field(&mut state, #field_idents_str, &self.entity.#field_idents)?; )*

                    stapifaction::serde::SerializeStruct::end(state)
                }
            }

            impl stapifaction::Persistable for #ident {
                fn path(&self) -> Option<std::path::PathBuf> {
                    #path
                }

                fn expand_strategy(&self) -> stapifaction::ExpandStrategy {
                    stapifaction::ExpandStrategy::SubsetsInSeparateFolders
                }

                fn serializable_entity<'e>(&'e self) -> Option<Box<dyn stapifaction::serde::ErasedSerialize + 'e>> {
                    let container = #container_ident { entity: self };

                    Some(Box::new(container) as Box<dyn stapifaction::serde::ErasedSerialize>)
                }

                fn children<'e>(&'e self) -> Box<dyn Iterator<Item = (Option<std::path::PathBuf>, std::borrow::Cow<'e, stapifaction::Child<'e>>)> + 'e>
                {
                    let mut map = std::collections::HashMap::new();

                    #(
                        map.insert(Some(#subset_path_buf),
                         std::borrow::Cow::Owned(stapifaction::Child::subset(&self.#subset_idents))
                        );
                    )*

                    #(
                        if let Some(subset) = &self.#optional_subset_idents {
                            map.insert(Some(#optional_subset_path_buf),
                                std::borrow::Cow::Owned(stapifaction::Child::subset(subset))
                            );
                        }
                    )*

                    #(
                        map.insert(Some(#collection_path_buf),
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
#[darling(attributes(persistable), supports(struct_any))]
pub struct PersistableInputReceiver {
    pub path: Option<String>,
}

#[derive(Debug, FromField)]
#[darling(attributes(persistable))]
pub struct PersistableField {
    pub ident: Option<Ident>,
    pub ty: Type,
    #[darling(default)]
    pub id: bool,
    pub expand: Option<Override<Expand>>,
}

impl PersistableField {
    pub fn expand(&self) -> Option<Cow<'_, Expand>> {
        match &self.expand {
            Some(expand) => match expand {
                Override::Explicit(value) => Some(Cow::Borrowed(value)),
                Override::Inherit => Some(Cow::Owned(Expand::Subset)),
            },
            None => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, FromMeta)]
#[darling(default)]
pub enum Expand {
    #[default]
    Subset,
    All,
}

#[cfg(test)]
mod tests {
    use darling::FromDeriveInput;
    use syn::parse_quote;

    use crate::persistable_input_receiver::PersistableInputReceiver;

    #[test]
    fn test_persistable_entity() {
        let di = parse_quote! {
            #[derive(Persistable)]
            #[persistable(path = "users")]
            pub struct User {
                #[persistable(id)]
                user_name: String,
                first_name: String,
                last_name: String,
            }
        };

        let receiver = PersistableInputReceiver::from_derive_input(&di).unwrap();

        assert_eq!(receiver.path.unwrap(), "users");
    }
}
