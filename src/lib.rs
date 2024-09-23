use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, Ident, ItemFn, ItemStruct, Type};

#[proc_macro_attribute]
pub fn constructor(_args: TokenStream, source_input: TokenStream) -> TokenStream {
    let input = source_input.clone();
    let input = parse_macro_input!(input as ItemStruct);

    let mut fields_ident: Vec<Ident> = Vec::new();
    let mut fields_type: Vec<Type> = Vec::new();

    match &input.fields {
        Fields::Named(named) => {
            for i in &named.named {
                fields_ident.push(i.ident.clone().unwrap());
                fields_type.push(i.ty.clone());
            }
        }
        _ => return source_input,
    }

    let struct_ident = &input.ident;

    let struct_generics = &input.generics;

    let constructor = match &input.fields {
        Fields::Named(_) => quote! {
            fn #struct_ident(#(#fields_ident: #fields_type),*) -> #struct_ident {
                #struct_ident {
                    #(#fields_ident),*
                }
            }
        },
        Fields::Unnamed(_) => quote! {},
        Fields::Unit => quote! {
            fn #struct_ident() -> #struct_ident {
                #struct_ident
            }
        },
    }
    .into();

    let mut constructor = parse_macro_input!(constructor as ItemFn);

    constructor.sig.generics = struct_generics.to_owned();

    quote! {
        #input

        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #constructor
    }
    .into()
}
