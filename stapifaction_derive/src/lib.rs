use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::persistable_input_receiver::PersistableInputReceiver;

mod persistable_input_receiver;

#[proc_macro_derive(Persistable, attributes(persistable))]
pub fn persistable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let receiver = PersistableInputReceiver::from_derive_input(&input).unwrap();

    let expanded = quote! { #receiver };

    TokenStream::from(expanded)
}
