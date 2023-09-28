use genco::prelude::*;

fn main() -> anyhow::Result<()> {
    let hash_map = rust::import("std::collections", "HashMap");

    let tokens: rust::Tokens = quote! {
        fn main() {
            let mut m = $hash_map::new();
            m.insert(1u32, 2u32);
        }
    };

    println!("{}", tokens.to_file_string()?);
    Ok(())
}
