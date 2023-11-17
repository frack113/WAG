//
// Tools box
//
// std::process::exit need a i32
pub const EXIST_ALL_GOOD: i32 = 0;
pub const EXIST_CLI_ERROR: i32 = 10;
pub const EXIST_TEST_ERROR: i32 = 11;

use std::collections::HashSet;

use regex_generate::{Generator, DEFAULT_MAX_REPEAT};
use windows::Win32::UI::Shell::IsUserAnAdmin;

// File Creation
pub fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 == 0 {
        (0..s.len())
            .step_by(2)
            .map(|i| {
                s.get(i..i + 2)
                    .and_then(|sub| u8::from_str_radix(sub, 16).ok())
            })
            .collect()
    } else {
        None
    }
}

/*
Some usefull fn
*/
pub fn regex_to_string(name: &String) -> String {
    let mut gen: Generator<rand::rngs::ThreadRng> =
        Generator::new(name, rand::thread_rng(), DEFAULT_MAX_REPEAT).unwrap();
    let mut buffer: Vec<u8> = vec![];
    gen.generate(&mut buffer).unwrap();
    let output: String = String::from_utf8(buffer).unwrap();

    return output;
}

pub fn process_is_admin() -> bool {
    return unsafe { IsUserAnAdmin().into() };
}

pub fn pretty_print_hashset(title: String, data: HashSet<String>) {
    println!("{} :", title);
    println!("----------------");
    for name in data {
        println!("👉 {}", name);
    }
    println!("----------------");
}