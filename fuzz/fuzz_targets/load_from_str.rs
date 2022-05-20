#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: (bool, bool, &str)| {
    let opts = ini::ParseOption {
        enabled_quote: input.0,
        enabled_escape: input.1,
    };

    if let Ok(conf) = ini::Ini::load_from_str_opt(input.2, opts) {
        for (a, b) in conf.iter() {
            println!("{a:?} {b:?}")
        }
    }
});
