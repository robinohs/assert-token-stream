use assert_tokenstreams_eq::assert_tokenstreams_eq;
use quote::quote;

fn main() {
    let first = quote! {
        fn test() {
            let x = 5;
            let z = 5;
        }
    };
    let second = quote! {
        fn test2() {
            let x = 5;
            let y = 5;
        }
    };
    assert_tokenstreams_eq!(&first, &second);
}
