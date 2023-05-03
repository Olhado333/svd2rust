#![no_main]
use libfuzzer_sys::fuzz_target;

use svd2rust::{generate, util::Config};

fuzz_target!(|data: &[u8]| {
    let input = String::from_utf8_lossy(data);
    let config = Config::default();
    let _ = generate(&input, &config);
});