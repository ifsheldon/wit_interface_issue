wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "formatter",
});

use std::panic;

struct Formatter;

impl Guest for Formatter {
    fn format_str(a: String, b: String) -> String {
        panic::set_hook(Box::new(|_panic_info| {
            // do nothing
        }));
        let s = format!("{} + {}", a, b);
        print(s.as_str());
        let env = std::env::var("SOME_VAR").unwrap_or_else(|_| "No SOME_VAR".to_string());
        print(env.as_str());
        s
    }
}

export!(Formatter);