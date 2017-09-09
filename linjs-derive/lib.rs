/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate proc_macro;
extern crate syn;
extern crate synstructure;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

//  -------------------------------------------------------------------------------------------------------

#[proc_macro_derive(JSRootable)]
pub fn derive_js_rootable(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_js_rootable(&ast);
    gen.parse().unwrap()
}

fn impl_js_rootable(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let (_, ty_generics, _) = ast.generics.split_for_impl();

    let impl_generics = ast.generics.ty_params.iter().map(|ty| quote! { #ty });
    let impl_generics = quote! { #(#impl_generics),* };

    // append the lifetime constraints to the generic type parameters
    let lifetime_constraints = ast.generics.ty_params.iter().map(|ty| {
        let ident = &ty.ident;
        quote! { #ident: 'b }
    });

    let where_clause_predicates = ast.generics.where_clause.predicates.iter().map(|pred| quote! { #pred });
    let where_clause_items = lifetime_constraints.chain(where_clause_predicates).collect::<Vec<_>>();
    let where_clause = if where_clause_items.is_empty() {
        quote! { }
    } else {
        quote! { where #(#where_clause_items),* }
    };

    // For types without any liftime parameters, we provide a trivial
    // implementation of `JSRootable`.
    if ast.generics.lifetimes.is_empty() {
        return quote! {
            #[allow(unsafe_code)]
            unsafe impl<'a, #impl_generics> ::linjs::JSRootable<'a> for #name #ty_generics #where_clause {
                type Aged = #name #ty_generics;
            }
        }
    }

    // we assume there's only one lifetime param, not named 'b
    assert!(ast.generics.lifetimes.len() == 1, "deriving JSRootable requires a single lifetime");

    let impl_lifetime = &ast.generics.lifetimes[0].lifetime.ident;
    assert!(impl_lifetime != "'b", "deriving JSRootable requires the lifetime to not be named 'b");

    // the `Aged` associated type params are the ty_params without their bounds
    let aged_ty_params = ast.generics.ty_params.iter().map(|ty| {
        let ident = &ty.ident;
        quote! { #ident }
    });
    let aged_ty_params = quote! { #(#aged_ty_params),* };

    quote! {
        #[allow(unsafe_code)]
        unsafe impl<#impl_lifetime, 'b, #impl_generics> ::linjs::JSRootable<'b> for #name #ty_generics #where_clause {
            type Aged = #name<'b, #aged_ty_params>;
        }
    }
}

//  -------------------------------------------------------------------------------------------------------

#[proc_macro_derive(JSTraceable)]
pub fn expand_token_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand_string(&input.to_string()).parse().unwrap()
}

fn expand_string(input: &str) -> String {
    let mut type_ = syn::parse_macro_input(input).unwrap();

    let style = synstructure::BindStyle::Ref.into();
    let match_body = synstructure::each_field(&mut type_, &style, |binding| {
        Some(quote! { #binding.trace(tracer); })
    });

    let name = &type_.ident;
    let (impl_generics, ty_generics, where_clause) = type_.generics.split_for_impl();
    let mut where_clause = where_clause.clone();
    for param in type_.generics.ty_params.iter().skip(1) {
        where_clause.predicates.push(syn::WherePredicate::BoundPredicate(syn::WhereBoundPredicate {
            bound_lifetimes: Vec::new(),
            bounded_ty: syn::Ty::Path(None, param.ident.clone().into()),
            bounds: vec![syn::TyParamBound::Trait(
                syn::PolyTraitRef {
                    bound_lifetimes: Vec::new(),
                    trait_ref: syn::parse_path("::linjs::JSTraceable").unwrap(),
                },
                syn::TraitBoundModifier::None
            )],
        }))
    }

    let tokens = quote! {
        #[allow(unsafe_code)]
        unsafe impl #impl_generics ::linjs::JSTraceable for #name #ty_generics #where_clause {
            #[inline]
            #[allow(unused_variables, unused_imports)]
            unsafe fn trace(&self, tracer: *mut ::linjs::JSTracer) {
                use ::linjs::JSTraceable;
                match *self {
                    #match_body
                }
            }
        }
    };

    tokens.to_string()
}
