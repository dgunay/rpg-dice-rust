#![no_main]
use libfuzzer_sys::fuzz_target;

use rpg_dice_rust::lib::solve_dice_expression;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = solve_dice_expression(String::from(s), None);
    }
});
