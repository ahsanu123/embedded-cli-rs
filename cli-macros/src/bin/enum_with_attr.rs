use cli_macros::MainCommand;
use darling::{util::parse_expr, FromDeriveInput, FromMeta};
use syn::{parse_quote, Expr};

struct CliHandle;

impl CliHandle {
    pub fn hello() {
        todo!()
    }
}

impl CliHandle {
    pub fn second_hello() {
        todo!()
    }
}

fn main() {
    let main_command_stmt: syn::DeriveInput = parse_quote! {
        #[derive(MainCommand)]
        enum BaseCommand {
            Status,
            Led { pin: u8 },
            Adc { ping: u8, time: u8 },
        }
    };

    // let stmt: syn::DeriveInput = parse_quote! {
    //     #[demo(example1 = test::path, example2 = "hello")]
    //     struct Example;
    // };
    //
    // println!("{:#?}", stmt);
    //
    // let input = Receiver::from_derive_input(&parse_quote! {
    //     #[demo(example1 = test::path, example2 = "hello")]
    //     struct Example;
    // })
    // .unwrap();

    // assert_eq!(input.example1, Some(parse_quote!(test::path)));
    // assert_eq!(input.example2, "HELLO".to_string());
}
