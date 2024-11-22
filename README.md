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

## Tracking Issue

https://github.com/rust-lang/rust/issues/133235

## The fix

The [comment](https://github.com/rust-lang/rust/issues/133235#issuecomment-2491533929) by Alex Crichton.

Also see the [`run.sh` script](https://github.com/ifsheldon/wit_interface_issue/blob/try-fix/host/run.sh) in the try-fix branch. 