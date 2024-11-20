wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "formatter",
});

struct Formatter;

impl Guest for Formatter {
    fn format_str(a: String, b: String) -> String {
        let s = format!("{} + {}", a, b);
        print(s.as_str());
        s
    }
}

export!(Formatter);