#![allow(unused)]
mod bpe;

use bpe::prelude::*;
use env_logger::Builder;

fn main() {
    Builder::from_default_env()
        .format_timestamp(None)
        // .format_module_path(false)
        // .format_source_path(false)
        .format_target(false)
        .init();

    let mut token = Bpe::new();
    token.read_string("aaabdaaabac".to_string());
    token.show_original_string();
    token.set_max_tokens(2);
    token.tokenize();
    token.show_difference();
}
