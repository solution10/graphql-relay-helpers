use proc_macro2::{Ident, Span};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(RelayConnection)]
pub fn macro_relay_connection_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let out = match input.data {
        Data::Struct(_s) => {
            let connection_name = Ident::new(&format!("{}_Connection", input.ident), Span::mixed_site());
            let edge_name = Ident::new(&format!("{}_Edge", input.ident), Span::mixed_site());
            let struct_name = input.ident;

            quote! {
                #[derive(Debug)]
                pub struct #connection_name {
                    pub count: i32,
                    pub edges: Vec<#edge_name>,
                }

                #[derive(Debug)]
                pub struct #edge_name {
                    pub node: #struct_name,
                    pub cursor: Option<String>,
                }
            }
        }
        _ => todo!() // Actually this should throw.
    };

    out.into()
}
