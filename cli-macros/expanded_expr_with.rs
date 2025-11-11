#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use darling::{util::parse_expr, FromDeriveInput, FromMeta};
use syn::{parse_quote, Expr};

#[darling(attributes(demo))]
pub struct Receiver {
    #[darling(with = parse_expr::preserve_str_literal, map = Some)]
    example1: Option<Expr>,
    #[darling(with = |m|Ok(String::from_meta(m)?.to_uppercase()), default)]
    example2: String,
}
#[automatically_derived]
#[allow(clippy::manual_unwrap_or_default)]
impl ::darling::FromDeriveInput for Receiver {
    fn from_derive_input(__di: &::darling::export::syn::DeriveInput) -> ::darling::Result<Self> {
        let mut __errors = ::darling::Error::accumulator();
        let mut example1: (bool, ::darling::export::Option<Option<Expr>>) = (false, None);
        let mut example2: (bool, ::darling::export::Option<String>) = (false, None);
        use ::darling::ToTokens;
        for __attr in &__di.attrs {
            match ::darling::export::ToString::to_string(&__attr.path().clone().into_token_stream())
                .as_str()
            {
                "demo" => match ::darling::util::parse_attribute_to_meta_list(__attr) {
                    ::darling::export::Ok(__data) => {
                        match ::darling::export::NestedMeta::parse_meta_list(__data.tokens) {
                            ::darling::export::Ok(ref __items) => {
                                if __items.is_empty() {
                                    continue;
                                }
                                for __item in __items {
                                    match *__item {
                                        ::darling::export::NestedMeta::Meta(ref __inner) => {
                                            let __name =
                                                ::darling::util::path_to_string(__inner.path());
                                            match __name.as_str() {
                                                "example1" => {
                                                    if !example1.0 {
                                                        example1 = (
                                                                true,
                                                                __errors
                                                                    .handle(
                                                                        ::darling::export::identity::<
                                                                            fn(&::darling::export::syn::Meta) -> ::darling::Result<_>,
                                                                        >(parse_expr::preserve_str_literal)(__inner)
                                                                            .map(Some)
                                                                            .map_err(|e| e.with_span(&__inner).at("example1")),
                                                                    ),
                                                            );
                                                    } else {
                                                        __errors.push(
                                                            ::darling::Error::duplicate_field(
                                                                "example1",
                                                            )
                                                            .with_span(&__inner),
                                                        );
                                                    }
                                                }
                                                "example2" => {
                                                    if !example2.0 {
                                                        example2 = (
                                                                true,
                                                                __errors
                                                                    .handle(
                                                                        ::darling::export::identity::<
                                                                            fn(&::darling::export::syn::Meta) -> ::darling::Result<_>,
                                                                        >(|m| Ok(String::from_meta(m)?.to_uppercase()))(__inner)
                                                                            .map_err(|e| e.with_span(&__inner).at("example2")),
                                                                    ),
                                                            );
                                                    } else {
                                                        __errors.push(
                                                            ::darling::Error::duplicate_field(
                                                                "example2",
                                                            )
                                                            .with_span(&__inner),
                                                        );
                                                    }
                                                }
                                                __other => {
                                                    __errors.push(
                                                        ::darling::Error::unknown_field_with_alts(
                                                            __other,
                                                            &["example1", "example2"],
                                                        )
                                                        .with_span(__inner),
                                                    );
                                                }
                                            }
                                        }
                                        ::darling::export::NestedMeta::Lit(ref __inner) => {
                                            __errors.push(
                                                ::darling::Error::unsupported_format("literal")
                                                    .with_span(__inner),
                                            );
                                        }
                                    }
                                }
                            }
                            ::darling::export::Err(__err) => {
                                __errors.push(__err.into());
                            }
                        }
                    }
                    ::darling::export::Err(__err) => {
                        __errors.push(__err);
                    }
                },
                _ => continue,
            }
        }
        __errors.handle(::darling::export::Ok(&__di.data).and_then(::darling::export::Ok));
        if !example1.0 {
            match <Option<Expr> as ::darling::FromMeta>::from_none() {
                ::darling::export::Some(__type_fallback) => {
                    example1.1 = ::darling::export::Some(__type_fallback);
                }
                ::darling::export::None => {
                    __errors.push(::darling::Error::missing_field("example1"))
                }
            }
        }
        __errors.finish()?;
        ::darling::export::Ok(Receiver {
            example1: example1
                .1
                .expect("Uninitialized fields without defaults were already checked"),
            example2: if let Some(__val) = example2.1 {
                __val
            } else {
                ::darling::export::Default::default()
            },
        })
    }
}
fn main() {
    let input = Receiver::from_derive_input(&::syn::__private::parse_quote({
        let mut _s = ::quote::__private::TokenStream::new();
        ::quote::__private::push_pound(&mut _s);
        ::quote::__private::push_group(&mut _s, ::quote::__private::Delimiter::Bracket, {
            let mut _s = ::quote::__private::TokenStream::new();
            ::quote::__private::push_ident(&mut _s, "demo");
            ::quote::__private::push_group(&mut _s, ::quote::__private::Delimiter::Parenthesis, {
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::__private::push_ident(&mut _s, "example1");
                ::quote::__private::push_eq(&mut _s);
                ::quote::__private::push_ident(&mut _s, "test");
                ::quote::__private::push_colon2(&mut _s);
                ::quote::__private::push_ident(&mut _s, "path");
                ::quote::__private::push_comma(&mut _s);
                ::quote::__private::push_ident(&mut _s, "example2");
                ::quote::__private::push_eq(&mut _s);
                ::quote::__private::parse(&mut _s, "\"hello\"");
                _s
            });
            _s
        });
        ::quote::__private::push_ident(&mut _s, "struct");
        ::quote::__private::push_ident(&mut _s, "Example");
        ::quote::__private::push_semi(&mut _s);
        _s
    }))
    .unwrap();
    match (
        &input.example1,
        &Some(::syn::__private::parse_quote({
            let mut _s = ::quote::__private::TokenStream::new();
            ::quote::__private::push_ident(&mut _s, "test");
            ::quote::__private::push_colon2(&mut _s);
            ::quote::__private::push_ident(&mut _s, "path");
            _s
        })),
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    match (&input.example2, &"HELLO".to_string()) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
