use proc_macro2::{Ident, Span};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

/// Macro that will generate Connection and Edge structs for you to use when returning lists.
#[proc_macro_derive(RelayConnection)]
pub fn macro_relay_connection_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let out = match input.data {
        Data::Struct(_s) => {
            let connection_gql_name = format!("{}Connection", input.ident);
            let connection_gql_desc = format!("Connection type for {}.", input.ident);
            let edge_gql_name = format!("{}Edge", input.ident);
            let edge_gql_desc = format!("Edge type for {}.", input.ident);
            let connection_name = Ident::new(&format!("{}RelayConnection", input.ident), Span::mixed_site());
            let edge_name = Ident::new(&format!("{}RelayEdge", input.ident), Span::mixed_site());
            let struct_name = input.ident;

            quote! {
                #[derive(juniper::GraphQLObject, Debug, Clone, Eq, PartialEq)]
                #[graphql(
                    name = #connection_gql_name,
                    description = #connection_gql_desc
                )]
                pub struct #connection_name {
                    pub count: i32,
                    pub edges: Vec<#edge_name>,
                    pub page_info: PageInfo,
                }

                #[derive(juniper::GraphQLObject, Debug, Clone, Eq, PartialEq)]
                #[graphql(
                    name = #edge_gql_name,
                    description = #edge_gql_desc
                )]
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
