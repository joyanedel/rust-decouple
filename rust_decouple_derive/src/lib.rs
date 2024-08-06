use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Type};

#[proc_macro_derive(EnvVarParser)]
pub fn derive_env_var_parser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = match input.data {
        syn::Data::Struct(ref data_struct) => {
            let fields: Vec<_> = data_struct
                .fields
                .iter()
                .filter_map(|f| {
                    let ident = f.clone().ident;
                    let ty = f.ty.clone();
                    let ty_is_vec = f.ty.clone();
                    if ident.as_ref().is_some() {
                        Some((ident.unwrap(), ty, is_vec(ty_is_vec)))
                    } else {
                        None
                    }
                })
                .collect();
            let non_vec_fields: Vec<_> = fields.iter().filter(|(_, _, v)| !v).collect();
            let non_vec_fields = gen_fields(non_vec_fields);
            let vec_fields: Vec<_> = fields.iter().filter(|(_, _, v)| *v).collect();
            let vec_fields = gen_fields(vec_fields);

            quote! {
                impl EnvVarParser for #name {
                    fn parse() -> Self {
                        Self {
                            #non_vec_fields
                            #vec_fields
                        }
                    }
                }
            }
        }
        _ => panic!("#[derive(EnvVarParser)] is only defined for structs"),
    };

    expanded.into()
}

/// this function must be improved
fn is_vec(v: Type) -> bool {
    let v = quote!(#v).to_string();
    v.starts_with("Vec")
}

fn gen_fields(fields: Vec<&(Ident, Type, bool)>) -> proc_macro2::TokenStream {
    if fields.len() == 0 {
        return quote! {};
    }

    let field_names: Vec<_> = fields.iter().map(|(f, _, _)| f).collect();
    let field_names_uppercase: Vec<_> = fields
        .iter()
        .map(|(f, _, _)| f.to_string().to_uppercase())
        .collect();
    let field_types: Vec<_> = fields.iter().map(|(_, t, _)| t).collect();
    let is_vec = fields.iter().next().unwrap().2;

    if is_vec {
        quote! {
            #(#field_names: rust_decouple::core::VecEnvironment::from(#field_names_uppercase, None) as #field_types,)*
        }
    } else {
        quote! {
            #(#field_names: rust_decouple::core::Environment::from::<#field_types>(#field_names_uppercase, None),)*
        }
    }
}
