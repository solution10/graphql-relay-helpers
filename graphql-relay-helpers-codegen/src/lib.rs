use proc_macro2::{Ident, Span};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

/// Macro that will generate Connection and Edge structs for you to use when returning lists.
///
/// The object that this is applied to must also #[derive(juniper::GraphQLObject)] otherwise you'll
/// get a compilation error.
///
/// Given the following struct:
///
/// ```
/// #[derive(juniper::GraphQLObject, RelayConnection)]
/// struct PlayableCharacter {
///     pub name: String,
///     pub theme_song: String,
/// }
/// ```
///
/// The `RelayConnection` macro will generate two additional structs:
///
/// ```
/// #[derive(juniper::GraphQLObject)]
/// struct PlayableCharacterRelayConnection {
///     count: usize,
///     edges: Vec<PlayableCharacterRelayEdge>
///     page_info: PageInfo
/// }
///
/// #[derive(juniper::GraphQLObject)]
/// struct PlayableCharacterRelayEdge {
///     cursor: String,
///     node: PlayableCharacter,
/// }
///
/// ```
///
///
#[proc_macro_derive(RelayConnection)]
pub fn macro_relay_connection_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let out = match input.data {
        Data::Struct(_s) => {
            let connection_name = Ident::new(&format!("{}RelayConnection", input.ident), Span::mixed_site());
            let edge_name = Ident::new(&format!("{}RelayEdge", input.ident), Span::mixed_site());
            let struct_name = input.ident;

            quote! {
                #[derive(Debug, juniper::GraphQLObject)]
                pub struct #connection_name {
                    pub count: i32,
                    pub edges: Vec<#edge_name>,
                    pub page_info: PageInfo,
                }

                #[derive(Debug, juniper::GraphQLObject)]
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
