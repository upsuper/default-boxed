extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput, Data, Fields, Type};

#[proc_macro_derive(DefaultBoxed)]
pub fn derive_default_boxed(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive(syn::parse_macro_input!(input)).into()
}

fn derive(mut input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let data = match input.data {
        Data::Struct(data) => data,
        _ => panic!("only structs are supported"),
    };
    let fields = match data.fields {
        Fields::Named(fields) => fields,
        _ => panic!("only structs with named fields are supported"),
    };
    let body: TokenStream = fields.named.into_iter().map(|field| {
        let name = field.ident.unwrap();
        match field.ty {
            Type::Array(array) => {
                let ty = array.elem;
                let len = array.len;
                quote! {
                    let #name = (*ptr).#name.as_mut_ptr();
                    for i in 0usize..#len {
                        <#ty as default_boxed::DefaultBoxed>::default_in_place(#name.offset(i as isize));
                    }
                }
            }
            ty => quote!{
                <#ty as default_boxed::DefaultBoxed>::default_in_place(&mut (*ptr).#name);
            }
        }
    }).collect();

    if !input.generics.params.is_empty() {
        let mut where_clause = input.generics.where_clause.take();
        let predicates = &mut where_clause.get_or_insert(parse_quote!(where)).predicates;
        for param in input.generics.type_params() {
            let ident = &param.ident;
            predicates.push(parse_quote!(#ident: default_boxed::DefaultBoxed));
        }
        input.generics.where_clause = where_clause;
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl#impl_generics default_boxed::DefaultBoxed for #name#ty_generics
        #where_clause
        {
            unsafe fn default_in_place(ptr: *mut Self) {
                #body
            }
        }
    }
}
