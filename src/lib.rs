mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn generate_text(data: &str, rule_length: usize, result_length: usize) -> String {
    let rule = markov_chain_text_generator::generate_rule_from_data(data, rule_length).unwrap();
    markov_chain_text_generator::generate_text(&rule, result_length)
}
