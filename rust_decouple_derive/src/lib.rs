use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericArgument, Ident, PathArguments, Type};

#[proc_macro_derive(Decouple)]
pub fn derive_env_var_parser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

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
                impl Decouple for #struct_name {
                    type Error = rust_decouple::core::FromEnvironmentError;
                    fn parse() -> Result<Self, Self::Error> where Self: Sized {
                        Ok(Self {
                            #non_vec_fields
                            #vec_fields
                        })
                    }
                }
            }
        }
        _ => panic!("#[derive(Decouple)] is only defined for structs"),
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
        let extracted_field_types: Vec<_> = field_types
            .iter()
            .map(|ty| extract_inner_type_from_vec(ty).unwrap())
            .collect();

        quote! {
            #(#field_names: (rust_decouple::core::VecEnvironment::from::<#extracted_field_types>(#field_names_uppercase, None))?,)*
        }
    } else {
        quote! {
            #(#field_names: (rust_decouple::core::Environment::from::<#field_types>(#field_names_uppercase, None))?,)*
        }
    }
}

// This function checks if a type is a Vec<T> and returns T if it is.
fn extract_inner_type_from_vec(ty: &Type) -> Option<&Type> {
    // Check if the type is a Path (which represents types like Vec, Option, etc.)
    if let Type::Path(type_path) = ty {
        // Check if the path segments are not empty and match "Vec"
        if let Some(path_segment) = type_path.path.segments.last() {
            if path_segment.ident == "Vec" {
                // Now we check the generic arguments (which would hold the inner type T)
                if let PathArguments::AngleBracketed(angle_bracketed_args) = &path_segment.arguments
                {
                    // Check if there is exactly one generic argument (Vec<T> has one)
                    if let Some(GenericArgument::Type(inner_type)) =
                        angle_bracketed_args.args.first()
                    {
                        return Some(inner_type); // Return the inner type T
                    }
                }
            }
        }
    }
    None
}
