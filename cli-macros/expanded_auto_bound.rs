#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use darling::{FromDeriveInput, FromMeta};
enum Volume {
    Whisper,
    Talk,
    Shout,
}
#[automatically_derived]
#[allow(clippy::manual_unwrap_or_default)]
impl ::darling::FromMeta for Volume {
    fn from_list(__outer: &[::darling::export::NestedMeta]) -> ::darling::Result<Self> {
        match __outer.len() {
            0 => ::darling::export::Err(::darling::Error::too_few_items(1)),
            1 => {
                if let ::darling::export::NestedMeta::Meta(ref __nested) = __outer[0] {
                    match ::darling::util::path_to_string(__nested.path()).as_ref() {
                        "whisper" => {
                            if let ::darling::export::syn::Meta::Path(_) = *__nested {
                                ::darling::export::Ok(Volume::Whisper)
                            } else {
                                ::darling::export::Err(
                                    ::darling::Error::unsupported_format("non-path"),
                                )
                            }
                        }
                        "talk" => {
                            if let ::darling::export::syn::Meta::Path(_) = *__nested {
                                ::darling::export::Ok(Volume::Talk)
                            } else {
                                ::darling::export::Err(
                                    ::darling::Error::unsupported_format("non-path"),
                                )
                            }
                        }
                        "shout" => {
                            if let ::darling::export::syn::Meta::Path(_) = *__nested {
                                ::darling::export::Ok(Volume::Shout)
                            } else {
                                ::darling::export::Err(
                                    ::darling::Error::unsupported_format("non-path"),
                                )
                            }
                        }
                        __other => {
                            ::darling::export::Err(
                                ::darling::Error::unknown_field_with_alts(
                                        __other,
                                        &["whisper", "talk", "shout"],
                                    )
                                    .with_span(__nested),
                            )
                        }
                    }
                } else {
                    ::darling::export::Err(
                        ::darling::Error::unsupported_format("literal"),
                    )
                }
            }
            _ => ::darling::export::Err(::darling::Error::too_many_items(1)),
        }
    }
    fn from_string(lit: &str) -> ::darling::Result<Self> {
        match lit {
            "whisper" => ::darling::export::Ok(Volume::Whisper),
            "talk" => ::darling::export::Ok(Volume::Talk),
            "shout" => ::darling::export::Ok(Volume::Shout),
            __other => {
                ::darling::export::Err(
                    ::darling::Error::unknown_value_with_alts(
                        __other,
                        &["whisper", "talk", "shout"],
                    ),
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Volume {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Volume {
    #[inline]
    fn eq(&self, other: &Volume) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Volume {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::fmt::Debug for Volume {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                Volume::Whisper => "Whisper",
                Volume::Talk => "Talk",
                Volume::Shout => "Shout",
            },
        )
    }
}
/// A more complex example showing the ability to skip at a field or struct
/// level while still tracking which type parameters need to be bounded.
/// This can be seen by expanding this example using `cargo expand`.
#[allow(dead_code)]
enum Emphasis<T> {
    Constant(Volume),
    Variable(darling::util::PathList),
    #[darling(skip)]
    PerPhoneme(Option<T>),
    Strided { #[darling(skip)] step: Vec<T>, #[darling(multiple)] volume: Vec<Volume> },
}
#[automatically_derived]
#[allow(clippy::manual_unwrap_or_default)]
impl<T> ::darling::FromMeta for Emphasis<T> {
    fn from_list(__outer: &[::darling::export::NestedMeta]) -> ::darling::Result<Self> {
        match __outer.len() {
            0 => ::darling::export::Err(::darling::Error::too_few_items(1)),
            1 => {
                if let ::darling::export::NestedMeta::Meta(ref __nested) = __outer[0] {
                    match ::darling::util::path_to_string(__nested.path()).as_ref() {
                        "constant" => {
                            ::darling::export::Ok(
                                Emphasis::Constant(
                                    ::darling::FromMeta::from_meta(__nested)
                                        .map_err(|e| e.at("constant"))?,
                                ),
                            )
                        }
                        "variable" => {
                            ::darling::export::Ok(
                                Emphasis::Variable(
                                    ::darling::FromMeta::from_meta(__nested)
                                        .map_err(|e| e.at("variable"))?,
                                ),
                            )
                        }
                        "strided" => {
                            if let ::darling::export::syn::Meta::List(ref __data) = *__nested {
                                let __items = ::darling::export::NestedMeta::parse_meta_list(
                                    __data.tokens.clone(),
                                )?;
                                let __items = &__items;
                                let mut __errors = ::darling::Error::accumulator();
                                let mut step: (bool, ::darling::export::Option<Vec<T>>) = (
                                    false,
                                    None,
                                );
                                let mut volume: Vec<Volume> = ::darling::export::Default::default();
                                for __item in __items {
                                    match *__item {
                                        ::darling::export::NestedMeta::Meta(ref __inner) => {
                                            let __name = ::darling::util::path_to_string(
                                                __inner.path(),
                                            );
                                            match __name.as_str() {
                                                "volume" => {
                                                    let __len = volume.len();
                                                    if let ::darling::export::Some(__val) = __errors
                                                        .handle(
                                                            ::darling::export::identity::<
                                                                fn(&::darling::export::syn::Meta) -> ::darling::Result<_>,
                                                            >(::darling::FromMeta::from_meta)(__inner)
                                                                .map_err(|e| {
                                                                    e
                                                                        .with_span(&__inner)
                                                                        .at(
                                                                            &::alloc::__export::must_use({
                                                                                ::alloc::fmt::format(
                                                                                    format_args!("{0}[{1}]", "volume", __len),
                                                                                )
                                                                            }),
                                                                        )
                                                                }),
                                                        )
                                                    {
                                                        volume.push(__val)
                                                    }
                                                }
                                                __other => {
                                                    __errors
                                                        .push(
                                                            ::darling::Error::unknown_field_with_alts(
                                                                    __other,
                                                                    &["volume"],
                                                                )
                                                                .with_span(__inner),
                                                        );
                                                }
                                            }
                                        }
                                        ::darling::export::NestedMeta::Lit(ref __inner) => {
                                            __errors
                                                .push(
                                                    ::darling::Error::unsupported_format("literal")
                                                        .with_span(__inner),
                                                );
                                        }
                                    }
                                }
                                __errors.finish().map_err(|e| e.at("strided"))?;
                                ::darling::export::Ok(Emphasis::Strided {
                                    step: if let Some(__val) = step.1 {
                                        __val
                                    } else {
                                        ::darling::export::Default::default()
                                    },
                                    volume: volume,
                                })
                            } else {
                                ::darling::export::Err(
                                    ::darling::Error::unsupported_format("non-list"),
                                )
                            }
                        }
                        __other => {
                            ::darling::export::Err(
                                ::darling::Error::unknown_field_with_alts(
                                        __other,
                                        &["constant", "variable", "per_phoneme", "strided"],
                                    )
                                    .with_span(__nested),
                            )
                        }
                    }
                } else {
                    ::darling::export::Err(
                        ::darling::Error::unsupported_format("literal"),
                    )
                }
            }
            _ => ::darling::export::Err(::darling::Error::too_many_items(1)),
        }
    }
    fn from_string(lit: &str) -> ::darling::Result<Self> {
        match lit {
            "constant" => {
                match <Volume as ::darling::FromMeta>::from_none() {
                    ::darling::export::Some(__value) => {
                        ::darling::export::Ok(Emphasis::Constant(__value))
                    }
                    ::darling::export::None => {
                        ::darling::export::Err(
                            ::darling::Error::unsupported_format("literal"),
                        )
                    }
                }
            }
            "variable" => {
                match <darling::util::PathList as ::darling::FromMeta>::from_none() {
                    ::darling::export::Some(__value) => {
                        ::darling::export::Ok(Emphasis::Variable(__value))
                    }
                    ::darling::export::None => {
                        ::darling::export::Err(
                            ::darling::Error::unsupported_format("literal"),
                        )
                    }
                }
            }
            "strided" => {
                ::darling::export::Err(::darling::Error::unsupported_format("literal"))
            }
            __other => {
                ::darling::export::Err(
                    ::darling::Error::unknown_value_with_alts(
                        __other,
                        &["constant", "variable", "per_phoneme", "strided"],
                    ),
                )
            }
        }
    }
}
#[darling(attributes(speak))]
struct SpeakingOptions<T, U> {
    max_volume: U,
    #[darling(skip, default)]
    additional_data: Vec<T>,
}
#[automatically_derived]
#[allow(clippy::manual_unwrap_or_default)]
impl<T, U: ::darling::FromMeta> ::darling::FromDeriveInput for SpeakingOptions<T, U> {
    fn from_derive_input(
        __di: &::darling::export::syn::DeriveInput,
    ) -> ::darling::Result<Self> {
        let mut __errors = ::darling::Error::accumulator();
        let mut max_volume: (bool, ::darling::export::Option<U>) = (false, None);
        let mut additional_data: (bool, ::darling::export::Option<Vec<T>>) = (
            false,
            None,
        );
        use ::darling::ToTokens;
        for __attr in &__di.attrs {
            match ::darling::export::ToString::to_string(
                    &__attr.path().clone().into_token_stream(),
                )
                .as_str()
            {
                "speak" => {
                    match ::darling::util::parse_attribute_to_meta_list(__attr) {
                        ::darling::export::Ok(__data) => {
                            match ::darling::export::NestedMeta::parse_meta_list(
                                __data.tokens,
                            ) {
                                ::darling::export::Ok(ref __items) => {
                                    if __items.is_empty() {
                                        continue;
                                    }
                                    for __item in __items {
                                        match *__item {
                                            ::darling::export::NestedMeta::Meta(ref __inner) => {
                                                let __name = ::darling::util::path_to_string(
                                                    __inner.path(),
                                                );
                                                match __name.as_str() {
                                                    "max_volume" => {
                                                        if !max_volume.0 {
                                                            max_volume = (
                                                                true,
                                                                __errors
                                                                    .handle(
                                                                        ::darling::export::identity::<
                                                                            fn(&::darling::export::syn::Meta) -> ::darling::Result<_>,
                                                                        >(::darling::FromMeta::from_meta)(__inner)
                                                                            .map_err(|e| e.with_span(&__inner).at("max_volume")),
                                                                    ),
                                                            );
                                                        } else {
                                                            __errors
                                                                .push(
                                                                    ::darling::Error::duplicate_field("max_volume")
                                                                        .with_span(&__inner),
                                                                );
                                                        }
                                                    }
                                                    __other => {
                                                        __errors
                                                            .push(
                                                                ::darling::Error::unknown_field_with_alts(
                                                                        __other,
                                                                        &["max_volume"],
                                                                    )
                                                                    .with_span(__inner),
                                                            );
                                                    }
                                                }
                                            }
                                            ::darling::export::NestedMeta::Lit(ref __inner) => {
                                                __errors
                                                    .push(
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
                    }
                }
                _ => continue,
            }
        }
        __errors
            .handle(::darling::export::Ok(&__di.data).and_then(::darling::export::Ok));
        if !max_volume.0 {
            match <U as ::darling::FromMeta>::from_none() {
                ::darling::export::Some(__type_fallback) => {
                    max_volume.1 = ::darling::export::Some(__type_fallback);
                }
                ::darling::export::None => {
                    __errors.push(::darling::Error::missing_field("max_volume"))
                }
            }
        }
        __errors.finish()?;
        ::darling::export::Ok(SpeakingOptions {
            max_volume: max_volume
                .1
                .expect("Uninitialized fields without defaults were already checked"),
            additional_data: if let Some(__val) = additional_data.1 {
                __val
            } else {
                ::darling::export::Default::default()
            },
        })
    }
}
struct Phoneme {
    #[allow(dead_code)]
    first: String,
}
#[automatically_derived]
impl ::core::default::Default for Phoneme {
    #[inline]
    fn default() -> Phoneme {
        Phoneme {
            first: ::core::default::Default::default(),
        }
    }
}
impl<T, U> Default for SpeakingOptions<T, U>
where
    Vec<T>: Default,
    U: Default,
{
    fn default() -> Self {
        Self {
            max_volume: Default::default(),
            additional_data: Default::default(),
        }
    }
}
fn main() {
    let derive_input = syn::parse_str(
            r#"
        #[derive(Speak)]
        #[speak(max_volume = "shout")]
        enum HtmlElement {
            Div(String)
        }
    "#,
        )
        .unwrap();
    let parsed: SpeakingOptions<Phoneme, Volume> = FromDeriveInput::from_derive_input(
            &derive_input,
        )
        .unwrap();
    match (&parsed.max_volume, &Volume::Shout) {
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
    match (&parsed.additional_data.len(), &0) {
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
