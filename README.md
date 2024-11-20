Why do we need:
* `sync::filesystem::types`
* `sync::io::streams`
* `cli::exit`
* `cli::environment`
* `cli::stdin`
* `cli::stdout`
* `cli::stderr`

to run a component that uses only std's "format!"?

## Reproduce
rustc: 1.82.0

1. `rustup target add wasm32-wasip2`
2. `cd host && sh run.sh`