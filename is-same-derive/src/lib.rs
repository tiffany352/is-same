extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::Data;
use syn::DeriveInput;
use syn::Fields;
use syn::Index;

#[proc_macro_derive(IsSame)]
pub fn derive_is_same(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let name = &input.ident;

    if let Data::Struct(data) = input.data {
        let fields = match data.fields {
            Fields::Named(fields) => {
                let fields = fields.named.iter().map(|field| {
                    let name = &field.ident;
                    quote! {
                        ::is_same::IsSame::is_same(&self.#name, &other.#name)
                    }
                });
                quote! {
                    #(#fields)&&*
                }
            }
            Fields::Unnamed(fields) => {
                let fields = fields.unnamed.iter().enumerate().map(|(index, _field)| {
                    let index = Index::from(index);
                    quote! {
                        ::is_same::IsSame::is_same(&self.#index, &other.#index)
                    }
                });
                quote! {
                    #(#fields)&&*
                }
            }
            Fields::Unit => quote!(true),
        };
        let tokens = quote! {
            impl ::is_same::IsSame for #name {
                fn is_same(&self, other: &Self) -> bool {
                    #fields
                }
            }
        };
        tokens.into()
    } else {
        panic!("derive(IsSame) can only be used with struct items")
    }
}
