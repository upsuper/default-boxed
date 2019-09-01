extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_quote, Data, DeriveInput, Fields, Index, Type};

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
        Fields::Named(fields) => fields.named.into_iter(),
        Fields::Unnamed(fields) => fields.unnamed.into_iter(),
        Fields::Unit => Punctuated::<_, Comma>::new().into_iter(),
    };
    let body: TokenStream = fields.enumerate().map(|(i, field)| {
        let name = field.ident.map_or_else(
            || Index::from(i).to_token_stream(),
            |ident| ident.to_token_stream(),
        );
        match field.ty {
            Type::Array(array) => {
                let ty = array.elem;
                let len = array.len;
                quote! {{
                    let arr = (*ptr).#name.as_mut_ptr();
                    for i in 0usize..#len {
                        <#ty as default_boxed::DefaultBoxed>::default_in_place(arr.offset(i as isize));
                    }
                }}
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
