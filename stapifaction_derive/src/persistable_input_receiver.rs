use std::{collections::HashSet, hash::Hash};

use darling::{FromDeriveInput, FromField};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use serde_derive_internals::ast::{Container, Data};
use syn::{Ident, Member, Type, TypePath};

pub fn expand_derive_persistable(serde_contrainer: Container) -> TokenStream {
    let Container {
        ident,
        data,
        original,
        ..
    } = serde_contrainer;
    let PersistableInputReceiver { path } =
        PersistableInputReceiver::from_derive_input(original).unwrap();

    let ident_str = ident.to_string();
    let container_ident = format_ident!("{}Container", ident);

    let path = match path {
        Some(path) => quote! { Some(String::from(#path).into()) },
        None => quote! { None},
    };

    if let Data::Struct(_, fields) = data {
        let fields = fields
            .into_iter()
            .map(|serde_field| {
                PersistableField::from_field(serde_field.original).map(|f| (serde_field, f))
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let (subsets, main_set) = fields.into_iter().partition::<Vec<_>, _>(|(_, f)| f.subset);

        let id = main_set.iter().find(|(_, f)| f.id).map(|(f, _)| &f.member);

        let id_ident = match &id {
            Some(id) => quote! { Some(format!("{}", self.#id).into()) },
            None => quote! { None },
        };

        let fields_count = main_set.len();
        let (field_idents_str, field_idents) = main_set
            .iter()
            .filter(|(f, _)| !f.attrs.skip_serializing())
            .filter_map(|(f, _)| match &f.member {
                Member::Named(ident) => Some((f.attrs.name().serialize_name(), ident)),
                Member::Unnamed(_) => None,
            })
            .multiunzip::<(Vec<_>, Vec<_>)>();

        let (subset_idents_str, subset_path_buf, subset_container_idents, subset_idents) = subsets
            .iter()
            .filter(|(f, _)| !f.attrs.skip_serializing())
            .filter_map(|(f, _)| match &f.member {
                Member::Named(ident) => Some((
                    f.attrs.name().serialize_name(),
                    build_subset_path_buf(&id, f.attrs.name().serialize_name()),
                    format_ident!("{}Container", type_ident(f.ty).unwrap()),
                    ident,
                )),
                Member::Unnamed(_) => None,
            })
            .sorted_by(|(a, _, _, _), (b, _, _, _)| a.cmp(b))
            .multiunzip::<(Vec<_>, Vec<_>, Vec<_>, Vec<_>)>();

        let duplicated_subsets = collect_duplicates(&subset_idents_str);

        if !duplicated_subsets.is_empty() {
            panic!(
                "The following subsets are duplicated: {}",
                duplicated_subsets.into_iter().join(", ")
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
                fn path() -> Option<std::path::PathBuf> {
                    #path
                }

                fn subsets<'a>(
                    &'a self,
                ) -> std::collections::HashMap<Option<std::path::PathBuf>, Box<dyn stapifaction::serde::ErasedSerialize + 'a>>
                {
                    let container = #container_ident { entity: self };
                    let mut map = std::collections::HashMap::new();

                    map.insert(#id_ident, Box::new(container) as Box<dyn stapifaction::serde::ErasedSerialize>);

                    #( map.insert(Some(#subset_path_buf), Box::new(#subset_container_idents { entity: &self.#subset_idents}) as Box<dyn stapifaction::serde::ErasedSerialize>); )*

                    map
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

fn type_ident(ty: &Type) -> Option<Ident> {
    if let Type::Path(type_path) = ty {
        type_path.path.segments.last().map(|s| s.ident.clone())
    } else {
        None
    }
}

fn build_subset_path_buf(id: &Option<&Member>, subset_name: &str) -> TokenStream {
    match id {
        Some(id) => {
            quote! { std::path::PathBuf::from(format!("{}/{}", self.#id, #subset_name))}
        }
        None => quote! { std::path::PathBuf::from(#subset_name) },
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
    #[darling(default)]
    pub subset: bool,
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
