use darling::{ast::Data, util::Ignored, FromDeriveInput, FromField};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Ident, Type};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(persistable), supports(struct_any))]
pub struct PersistableInputReceiver {
    pub ident: Ident,
    pub path: String,
    pub data: Data<Ignored, Field>,
}

impl ToTokens for PersistableInputReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let PersistableInputReceiver {
            ref ident,
            ref data,
            path,
        } = self;

        let ident_str = ident.to_string();
        let container_ident = format_ident!("{}Container", ident);

        if let Data::Struct(data) = data {
            let id = data
                .fields
                .iter()
                .find(|f| f.id)
                .expect("An id must be specified");
            let id_ident = id.ident.as_ref().expect("The id must be have an ident");
            let fields_count = data.len();
            let (field_idents_str, field_idents) = data
                .iter()
                .filter_map(|f| f.ident.as_ref())
                .map(|i| (i.to_string(), i))
                .multiunzip::<(Vec<_>, Vec<_>)>();

            tokens.extend(quote! {
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
            })
        } else {
            unimplemented!("Enums aren't supported");
        }
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(persistable))]
pub struct Field {
    pub ident: Option<Ident>,
    pub ty: Type,
    #[darling(default)]
    pub id: bool,
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

        assert_eq!(
            receiver
                .data
                .take_struct()
                .unwrap()
                .iter()
                .filter(|f| f.id)
                .count(),
            1
        );
        assert_eq!(receiver.path, "users");
    }
}
