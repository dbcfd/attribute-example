extern crate proc_macro;

use quote::{format_ident, quote};
//use quote::ToTokens;
use syn::{parse_macro_input, ImplItemMethod};

#[proc_macro_attribute]
pub fn sync_handler(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ImplItemMethod);

    let async_ident = format_ident!("{}_async", input.sig.ident);
    // eprintln!("{}", input.sig.to_token_stream());
    let invoke_method = input.sig.ident.clone();

    let ts = quote! {
        #input

        async fn #async_ident(&self) -> Result<(), std::io::Error> {
            self.#invoke_method()
        }
    };

    // eprintln!("Output={}", ts);

    ts.into()
}

#[proc_macro_attribute]
pub fn async_handler(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    input
}
