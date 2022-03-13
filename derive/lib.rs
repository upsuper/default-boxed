extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{parse_quote, Data, DeriveInput, Fields, Ident, Index, Type};

#[proc_macro_derive(DefaultBoxed)]
pub fn derive_default_boxed(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive(syn::parse_macro_input!(input)).into()
}

fn derive(mut input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let data = match input.data {
        Data::Struct(data) => data,
        _ => {
            return quote_spanned! { input.span() =>
                compile_error!("only structs are supported");
            };
        }
    };

    // Generate code to write default value into the position pointed by the ptr.
    let fields = match data.fields {
        Fields::Named(fields) => fields.named.into_iter(),
        Fields::Unnamed(fields) => fields.unnamed.into_iter(),
        Fields::Unit => Punctuated::<_, Comma>::new().into_iter(),
    };
    let write_default: TokenStream = fields
        .enumerate()
        .map(|(i, field)| {
            let name = field.ident.map_or_else(
                || Index::from(i).to_token_stream(),
                |ident| ident.to_token_stream(),
            );
            write_to_uninit(&quote!((*ptr).#name), &field.ty, 0)
        })
        .collect();

    if !input.generics.params.is_empty() {
        let mut where_clause = input.generics.where_clause.take();
        let predicates = &mut where_clause.get_or_insert(parse_quote!(where)).predicates;
        for param in input.generics.type_params() {
            let ident = &param.ident;
            predicates.push(parse_quote!(#ident: ::default_boxed::DefaultBoxed));
        }
        input.generics.where_clause = where_clause;
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        unsafe impl#impl_generics default_boxed::DefaultBoxed for #name#ty_generics
        #where_clause
        {
            unsafe fn default_in_place(ptr: *mut Self) {
                #write_default
            }
        }
    }
}

fn write_to_uninit(var: &TokenStream, ty: &Type, array_depth: usize) -> TokenStream {
    match ty {
        Type::Array(arr) => {
            let array_depth = array_depth + 1;
            let i = format!("i{array_depth}");
            let i = Ident::new(&i, Span::call_site());
            let len = &arr.len;
            let inner = write_to_uninit(&quote!(#var[#i]), &arr.elem, array_depth);
            quote! {
                for #i in 0..#len {
                    #inner
                }
            }
        }
        Type::Tuple(tuple) => tuple
            .elems
            .iter()
            .enumerate()
            .map(|(i, elem)| {
                let idx = Index::from(i);
                write_to_uninit(&quote!(#var.#idx), elem, array_depth)
            })
            .collect(),
        ty => {
            let call = quote_spanned! { ty.span() =>
                <#ty as ::default_boxed::DefaultBoxed>::default_in_place
            };
            quote! { #call(::core::ptr::addr_of_mut!(#var)); }
        }
    }
}
