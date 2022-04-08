use proc_macro::TokenStream;
use syn::{DeriveInput, parse};
use quote::quote;


#[proc_macro_derive(PackageData)]
pub fn my_macro_here_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(input).unwrap();
    let name = ast.ident;
    let gen = quote! {
        impl PackageData for #name {
            fn get_name(&self) -> &str {
                stringify!(#name)
            }
        }
    };
    gen.into()
}
