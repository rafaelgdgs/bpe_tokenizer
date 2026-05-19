#![allow(unused)]
mod bpe;

use std::time::Instant;

use bpe::prelude::*;
use env_logger::Builder;

fn main() {
    Builder::from_default_env()
        .format_timestamp(None)
        // .format_module_path(false)
        // .format_source_path(false)
        .format_target(false)
        .init();

    let start = Instant::now();

    let mut token = Bpe::new();
    // token.read_string("aaabdaaabac".to_string());
    token.read_file("data/dom_casmurro.txt");
    token.set_max_tokens(200);
    token.set_parallel(false);
    token.tokenize();

    let duration = start.elapsed();

    println!("Sequential results:");
    // token.show_original_string();
    // token.show_difference();
    println!("Execution time: {:?}", duration);
    println!();

    let start2 = Instant::now();

    let mut token_par = Bpe::new();
    // token_par.read_string("aaabdaaabac".to_string());
    token_par.read_file("data/dom_casmurro.txt");
    token_par.set_max_tokens(200);
    token_par.set_parallel(true);
    token_par.tokenize();

    let duration2 = start2.elapsed();

    println!("Parallel results:");
    // token_par.show_original_string();
    // token_par.show_difference();
    println!("Execution time: {:?}", duration2);
}
