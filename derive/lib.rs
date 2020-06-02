extern crate proc_macro;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_quote, Data, DeriveInput, Fields, GenericParam, Index, Type};

#[proc_macro_derive(DefaultBoxed)]
pub fn derive_default_boxed(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive(syn::parse_macro_input!(input)).into()
}

fn derive(mut input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let generics = &input.generics;
    let data = match input.data {
        Data::Struct(data) => data,
        _ => panic!("only structs are supported"),
    };

    // Generate a struct with all fields wrapped with `MaybeUninit`.
    let uninit_name = Ident::new(&format!("{}Uninit", name.to_string()), Span::call_site());
    let uninit_def = quote! { struct #uninit_name #generics };
    let uninit_struct = match &data.fields {
        Fields::Named(fields) => {
            let fields = fields.named.iter().map(|field| {
                let name = field.ident.as_ref().unwrap();
                let ty = wrap_maybe_uninit(&field.ty);
                quote! { #name: #ty }
            });
            quote! { #uninit_def { #(#fields,)* } }
        }
        Fields::Unnamed(fields) => {
            let fields = fields
                .unnamed
                .iter()
                .map(|field| wrap_maybe_uninit(&field.ty));
            quote! { #uninit_def (#(#fields),*); }
        }
        Fields::Unit => quote! { #uninit_def; },
    };

    // Generate the statement of a `MaybeUninit` struct of the given type.
    let params = if generics.params.is_empty() {
        quote! {}
    } else {
        let params = generics.params.iter().map(|param| match param {
            GenericParam::Type(ty) => ty.ident.to_token_stream(),
            GenericParam::Const(c) => c.ident.to_token_stream(),
            GenericParam::Lifetime(l) => l.lifetime.to_token_stream(),
        });
        quote! { <#(#params),*> }
    };
    let transmute_uninit = quote! {
        let uninit: &mut #uninit_name #params = ::core::mem::transmute(ptr);
    };

    // Generate code to write default value into the `MaybeUninit` reference.
    let fields = match data.fields {
        Fields::Named(fields) => fields.named.into_iter(),
        Fields::Unnamed(fields) => fields.unnamed.into_iter(),
        Fields::Unit => Punctuated::<_, Comma>::new().into_iter(),
    };
    let write_default: TokenStream = fields.enumerate().map(|(i, field)| {
        let name = field.ident.map_or_else(
            || Index::from(i).to_token_stream(),
            |ident| ident.to_token_stream(),
        );
        match field.ty {
            Type::Array(array) => {
                let ty = array.elem;
                quote! {{
                    for item in uninit.#name.iter_mut() {
                        <#ty as ::default_boxed::DefaultBoxed>::default_in_place(item.as_mut_ptr());
                    }
                }}
            }
            ty => quote!{
                <#ty as ::default_boxed::DefaultBoxed>::default_in_place(uninit.#name.as_mut_ptr());
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
                #uninit_struct
                #transmute_uninit
                #write_default
            }
        }
    }
}

/// Wrap the given type with `MaybeUninit`.
/// If the type is an array, wrap the inner type with `MaybeUninit`.
fn wrap_maybe_uninit(ty: &Type) -> TokenStream {
    match ty {
        Type::Array(arr) => {
            let elem = wrap_maybe_uninit(&arr.elem);
            let len = &arr.len;
            quote! { [#elem; #len] }
        }
        _ => quote! { ::core::mem::MaybeUninit<#ty> },
    }
}
