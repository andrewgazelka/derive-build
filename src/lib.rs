extern crate proc_macro;

use proc_macro::TokenStream;

use inflector::string::singularize::to_singular;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta, Path, Type, TypePath};

#[proc_macro_derive(Build, attributes(required, default))]
pub fn build_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_build_macro(&ast)
}

/// remove the `Into` trait from the type if it is an integer because
/// it makes the API less pretty (we have to explicitly state the integer type)
fn normalize(input: &Type) -> proc_macro2::TokenStream {
    match input {
        // Handle references differently to avoid using `Into` trait for them
        Type::Reference(r) => {
            quote! { #r }
        }
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) => {
            let last_segment = segments.last().unwrap();
            let ident = &last_segment.ident;

            // Handle integers differently to avoid using `Into` trait for them
            if ident == "i8"
                || ident == "i16"
                || ident == "i32"
                || ident == "i64"
                || ident == "i128"
                || ident == "isize"
                || ident == "u8"
                || ident == "u16"
                || ident == "u32"
                || ident == "u64"
                || ident == "u128"
                || ident == "usize"
            {
                quote! { #input }
            } else {
                quote! { impl Into<#input> }
            }
        }
        _ => {
            quote! { impl Into<#input> }
        }
    }
}

#[allow(clippy::too_many_lines)]
fn impl_build_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (required_fields, optional_fields) = partition_fields(&ast.data);

    let (optional_fields, optional_defaults): (Vec<_>, Vec<_>) = optional_fields
        .iter()
        .map(|field| {
            let Some(default_attr) = field
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("default")) else {
                    return (field, quote! { Default::default() });
            };

            let Meta::NameValue(v)= &default_attr.meta else {
                panic!("only named values allowed for default attribute")
            };

            let v = &v.value;
            let default_value = quote!(#v);

            (field, default_value)
        })
        .unzip();

    let required_params = required_fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let field_type = normalize(field_type);
        quote! { #field_name: #field_type }
    });

    let required_assignments = required_fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: #field_name.into() }
    });

    let optional_methods = optional_fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;

        if let syn::Type::Path(syn::TypePath {
            path: syn::Path { segments, .. },
            ..
        }) = field_type
        {
            if let Some(syn::PathSegment {
                ident,
                arguments:
                    syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        args, ..
                    }),
            }) = segments.first()
            {
                let singular: syn::Ident = {
                    let field_name_str = field_name.clone().unwrap().to_string();
                    let singular = to_singular(&field_name_str);
                    syn::parse_str(&singular).unwrap()
                };

                match ident.to_string().as_str() {
                    "Option" => {
                        if let Some(syn::GenericArgument::Type(inner_type)) = args.first() {
                            let inner_type = normalize(inner_type);
                            return quote! {
                                pub fn #field_name(mut self, #field_name: #inner_type) -> Self {
                                    self.#field_name = Some(#field_name.into());
                                    self
                                }
                            };
                        }
                    }
                    "Vec" => {
                        if let Some(syn::GenericArgument::Type(inner_type)) = args.first() {
                            let inner_type = normalize(inner_type);
                            return quote! {
                                pub fn #singular(mut self, #singular: #inner_type) -> Self {
                                    self.#field_name.push(#singular.into());
                                    self
                                }
                            };
                        }
                    }
                    "HashMap" | "BTreeMap" => {
                        let mut args = args.iter();
                        if let Some(syn::GenericArgument::Type(key_type)) = args.next() {
                            if let Some(syn::GenericArgument::Type(value_type)) = args.next() {
                                let key_type = normalize(key_type);
                                let value_type = normalize(value_type);
                                return quote! {
                                    pub fn #singular(mut self, key: #key_type, value: #value_type) -> Self {
                                        self.#field_name.insert(key.into(), value.into());
                                        self
                                    }
                                };
                            }
                        }
                    }
                    "HashSet" | "BTreeSet" => {
                        if let Some(syn::GenericArgument::Type(inner_type)) = args.first() {
                            let inner_type = normalize(inner_type);
                            return quote! {
                                pub fn #singular(mut self, #singular: #inner_type) -> Self {
                                    self.#field_name.insert(#singular.into());
                                    self
                                }
                            };
                        }
                    }
                    _ => {}
                }
            }
        }

        let field_type = normalize(field_type);
        quote! {
            pub fn #field_name(mut self, #field_name: #field_type) -> Self {
                self.#field_name = #field_name.into();
                self
            }
        }
    });

    let optional_field_idents = optional_fields.iter().map(|field| &field.ident);

    let expanded = quote! {
        impl #generics #name #generics {
            pub fn new(#(#required_params),*) -> Self {
                Self {
                    #(#required_assignments,)*
                    #(
                        #optional_field_idents: #optional_defaults,
                    )*
                }
            }

            #(#optional_methods)*
        }
    };

    TokenStream::from(expanded)
}

fn partition_fields(data: &syn::Data) -> (Vec<syn::Field>, Vec<syn::Field>) {
    let fields = match data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("Only structs are supported for the Build macro."),
    };

    fields.iter().cloned().partition(|field| {
        field
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("required"))
    })
}
