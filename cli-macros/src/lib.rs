use proc_macro::TokenStream;

mod derive_main_command;

#[proc_macro_derive(MainCommand)]
pub fn main_command(input: TokenStream) -> TokenStream {
    derive_main_command::derive(input)
}
