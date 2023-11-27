use darling::{FromDeriveInput, FromField};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde_derive_internals::ast::{Container, Data};
use syn::{Ident, Member, Type};

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

    if let Data::Struct(_, fields) = data {
        let fields = fields
            .into_iter()
            .map(|serde_field| {
                PersistableField::from_field(serde_field.original).map(|f| (serde_field, f))
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let (id, _) = fields
            .iter()
            .find(|(_, f)| f.id)
            .expect("An id must be specified");

        let id_ident = &id.member;
        let fields_count = fields.len();
        let (field_idents_str, field_idents) = fields
            .iter()
            .filter(|(f, _)| !f.attrs.skip_serializing())
            .filter_map(|(f, _)| match &f.member {
                Member::Named(ident) => Some((f.attrs.name().serialize_name(), ident)),
                Member::Unnamed(_) => None,
            })
            .multiunzip::<(Vec<_>, Vec<_>)>();

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
                fn path() -> std::path::PathBuf {
                    #path.parse().unwrap()
                }

                fn subsets<'a>(
                    &'a self,
                ) -> std::collections::HashMap<std::path::PathBuf, Box<dyn stapifaction::serde::ErasedSerialize + 'a>>
                {
                    let container = #container_ident { entity: self };
                    std::collections::HashMap::from([(
                        format!("{}", self.#id_ident).into(),
                        Box::new(container) as Box<dyn stapifaction::serde::ErasedSerialize>,
                    )])
                }
            }
        }
    } else {
        unimplemented!("Enums aren't supported");
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(persistable), supports(struct_any))]
pub struct PersistableInputReceiver {
    pub path: String,
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

        assert_eq!(receiver.path, "users");
    }
}
