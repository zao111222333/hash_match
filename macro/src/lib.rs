use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Obtain hash of literals (&'static str)
#[proc_macro]
pub fn hash_arm(input: TokenStream) -> TokenStream {
    #[inline]
    fn hash_one<T: std::hash::Hash + ?Sized>(t: &T) -> u64 {
        const HASHER: foldhash::fast::FixedState = foldhash::fast::FixedState::with_seed(41);
        use std::hash::{BuildHasher, Hasher};
        let mut hasher = HASHER.build_hasher();
        t.hash(&mut hasher);
        hasher.finish()
    }
    let input_lit = parse_macro_input!(input as LitStr);
    let hash_value = hash_one(&input_lit.value());
    quote! {#hash_value}.into()
}

#[test]
fn parse_str() {
    use syn::parse_str;
    let input = "\"wadwa\"";
    let input_lit: LitStr = parse_str(input).unwrap();
    assert_eq!("wadwa", &input_lit.value());
}
