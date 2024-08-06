use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(EnvVarParser)]
pub fn derive_env_var_parser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = match input.data {
        syn::Data::Struct(ref data_struct) => {
            let fields: Vec<_> = data_struct
                .fields
                .iter()
                .filter_map(|f| {
                    let f = f.clone();
                    if f.ident.is_some() {
                        Some((f.ident.unwrap(), f.ty))
                    } else {
                        None
                    }
                })
                .collect();

            let field_names: Vec<_> = fields.iter().map(|(a, _)| a).collect();
            let field_names_uppercase: Vec<_> = field_names
                .iter()
                .map(|f| f.to_string().to_uppercase())
                .collect();
            let field_types: Vec<_> = fields.iter().map(|(_, b)| b).collect();

            quote! {
                impl EnvVarParser for #name {
                    fn parse() -> Self {
                        Self {
                            #(#field_names: rust_decouple::core::Environment::from::<#field_types>(#field_names_uppercase, None),)*
                        }
                    }
                }
            }
        }
        _ => panic!("#[derive(EnvVarParser)] is only defined for structs"),
    };

    expanded.into()
}
