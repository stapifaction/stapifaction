use persist_input_receiver::expand_derive_persist;
use proc_macro::TokenStream;
use serde_derive_internals::{ast::Container, Ctxt, Derive};
use syn::{parse_macro_input, DeriveInput};

mod persist_input_receiver;

#[proc_macro_derive(Persist, attributes(persist))]
pub fn persistable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let cx = Ctxt::new();
    let serde_container = Container::from_ast(&cx, &input, Derive::Serialize)
        .expect("Failed to create Serde container");

    let expanded = expand_derive_persist(serde_container);

    cx.check().unwrap();

    TokenStream::from(expanded)
}
