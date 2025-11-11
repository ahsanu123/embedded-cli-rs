use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Ident, Type};

pub fn derive(input: TokenStream) -> TokenStream {
    let quoted = parse_macro_input!(input as DeriveInput);

    // println!("{:#?}", input);

    let DeriveInput { ident, data, .. } = quoted;

    let main_command_trait_name =
        format_ident!("{}Trait", ident.to_string().to_case(Case::UpperCamel));

    let data = if let Data::Enum(data) = data {
        data
    } else {
        panic!("derive main command only for enum");
    };

    let handler_and_args = data
        .variants
        .iter()
        .map(|item| {
            let handle = format_ident!("handle_{}", item.ident.to_string().to_case(Case::Snake));
            let args = item
                .fields
                .iter()
                .map(|field| field.ty.clone())
                .collect::<Vec<Type>>();

            (handle, args)
        })
        .collect::<Vec<(Ident, Vec<Type>)>>();

    let quoted_handler = handler_and_args
        .iter()
        .map(|(handle, args)| {
            quote! {
                fn #handle(&mut self, handle_fn: Option<fn(#(#args),*)>){
                    todo!()
                }
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let cli_handle_trait = handler_and_args.iter().map(|(handle, args)| {
        quote! {
            fn #handle(&mut self, handle_fn: Option<fn(#(#args),*)>);
        }
    });

    let quoted = quote! {
        struct CliHandle;

        trait #main_command_trait_name {
            #(#cli_handle_trait)*
        }

        impl #main_command_trait_name for CliHandle {
            #(#quoted_handler)*
        }
    };

    // println!("{:#?}", quoted.to_string());
    quoted.into()
}
