//! Fuzz target for testing with random inputs.
//!
//! # Getting Started
//! See the cargo-fuzz tutorial: <https://rust-fuzz.github.io/book/cargo-fuzz/tutorial.html>
//!
//! # Running This Target
//! ```bash
//! cargo +nightly fuzz run fuzz_example
//! ```

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|_data: &[u8]| {
    // fuzzed code goes here
});
