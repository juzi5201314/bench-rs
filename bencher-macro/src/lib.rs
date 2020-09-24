extern crate proc_macro;

use std::str::FromStr;

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Debug, FromMeta)]
struct Args {
    #[darling(default)]
    name: Option<String>,

    #[darling(default)]
    count: Option<usize>,

    #[darling(default)]
    no_test: Option<()>
}

#[proc_macro_attribute]
pub fn bench(attrs: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let func = syn::parse_macro_input!(item as syn::ItemFn);
    let func_name = &func.sig.ident;
    let func_attrs = &func.attrs;


    let args: Args = Args::from_list(&syn::parse_macro_input!(attrs as syn::AttributeArgs)).unwrap();
    let name = args.name.map(|s| s.to_token_stream()).unwrap_or(func_name.to_string().to_token_stream());
    let count = args.count.unwrap_or(1000).to_token_stream();
    let test = if args.no_test.is_some() {
        TokenStream::new()
    } else {
        TokenStream::from_str("#[test]").unwrap()
    };

    let bencher = if cfg!(feature = "track-allocator") {
        quote! {
            Bencher::new(#name, #count, 0)
        }
    } else {
        quote! {
            Bencher::new(#name, #count, 0, GLOBAL.counter(), GLOBAL.peak())
        }
    };

    (quote! {
        #test
        #(#func_attrs)*
        fn #func_name() {
            #func

            let mut bencher = #bencher;
            #func_name(&mut bencher);
            bencher.finish();
        }
    }).into()
}