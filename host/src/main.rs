use wasmtime::component::*;
use wasmtime::{Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiImpl, WasiView};
use anyhow::Result;
// reference: https://docs.rs/wasmtime/latest/wasmtime/component/bindgen_examples/_0_hello_world/index.html
// reference: https://docs.wasmtime.dev/examples-rust-wasi.html


bindgen!({
    path: "../implementation/wit/format.wit",
    world: "formatter",
});

struct MyState {
    // These two are required basically as a standard way to enable the impl of WasiView
    wasi_ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for MyState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

impl FormatterImports for MyState {
    fn print(&mut self, s: String) {
        println!("{}", s);
    }
}

/// copied from wasmtime_wasi::type_annotate, which is a private function
fn type_annotate<T: WasiView, F>(val: F) -> F
where
    F: Fn(&mut T) -> WasiImpl<&mut T>,
{
    val
}

fn main() -> Result<()> {
    let engine = Engine::default();
    let component = Component::from_file(
        &engine,
        "../implementation/target/wasm32-wasip2/release/implementation.wasm",
    )?;

    let mut linker = Linker::new(&engine);

    let ctx = wasmtime_wasi::WasiCtxBuilder::new().build();
    let state = MyState {
        wasi_ctx: ctx,
        table: ResourceTable::new(),
    };
    let mut store = Store::new(&engine, state);
    Formatter::add_to_linker(&mut linker, |s| s)?;
    
    // Note: 
    // The below block is copied from `wasmtime_wasi::add_to_linker_sync`.
    // Interfaces that are okay to be removed without causing a runtime error are commented out.
    // But why a "format!()" in the implementation needs `sync::filesystem::types`, `sync::io::streams`, `cli::exit`, `cli::environment`, `cli::stdin`, `cli::stdout`, `cli::stderr`?
    {
        let l = &mut linker;
        let closure = type_annotate::<MyState, _>(|t| WasiImpl(t));
        let options = wasmtime_wasi::bindings::sync::LinkOptions::default();
        // wasmtime_wasi::bindings::clocks::wall_clock::add_to_linker_get_host(l, closure)?;
        // wasmtime_wasi::bindings::clocks::monotonic_clock::add_to_linker_get_host(l, closure)?;
        wasmtime_wasi::bindings::sync::filesystem::types::add_to_linker_get_host(l, closure)?;
        wasmtime_wasi::bindings::filesystem::preopens::add_to_linker_get_host(l, closure)?;
        wasmtime_wasi::bindings::io::error::add_to_linker_get_host(l, closure)?;
        // wasmtime_wasi::bindings::sync::io::poll::add_to_linker_get_host(l, closure)?;
        wasmtime_wasi::bindings::sync::io::streams::add_to_linker_get_host(l, closure)?;
        // wasmtime_wasi::bindings::random::random::add_to_linker_get_host(l, closure)?;
        // wasmtime_wasi::bindings::random::insecure::add_to_linker_get_host(l, closure)?;
        // wasmtime_wasi::bindings::random::insecure_seed::add_to_linker_get_host(l, closure)?;
        wasmtime_wasi::bindings::cli::exit::add_to_linker_get_host(l, &options.into(), closure)?;
        wasmtime_wasi::bindings::cli::environment::add_to_linker_get_host(l, closure)?;
        wasmtime_wasi::bindings::cli::stdin::add_to_linker_get_host(l, closure)?;
        wasmtime_wasi::bindings::cli::stdout::add_to_linker_get_host(l, closure)?;
        wasmtime_wasi::bindings::cli::stderr::add_to_linker_get_host(l, closure)?;
        // wasmtime_wasi::bindings::cli::terminal_input::add_to_linker_get_host(l, closure)?;
        // wasmtime_wasi::bindings::cli::terminal_output::add_to_linker_get_host(l, closure)?;
        // wasmtime_wasi::bindings::cli::terminal_stdin::add_to_linker_get_host(l, closure)?;
        // wasmtime_wasi::bindings::cli::terminal_stdout::add_to_linker_get_host(l, closure)?;
        // wasmtime_wasi::bindings::cli::terminal_stderr::add_to_linker_get_host(l, closure)?;
        // wasmtime_wasi::bindings::sync::sockets::tcp::add_to_linker_get_host(l, closure)?;
        // wasmtime_wasi::bindings::sockets::tcp_create_socket::add_to_linker_get_host(l, closure)?;
        // wasmtime_wasi::bindings::sync::sockets::udp::add_to_linker_get_host(l, closure)?;
        // wasmtime_wasi::bindings::sockets::udp_create_socket::add_to_linker_get_host(l, closure)?;
        // wasmtime_wasi::bindings::sockets::instance_network::add_to_linker_get_host(l, closure)?;
        // wasmtime_wasi::bindings::sockets::network::add_to_linker_get_host(l, closure)?;
        // wasmtime_wasi::bindings::sockets::ip_name_lookup::add_to_linker_get_host(l, closure)?;
    }

    let bindings = Formatter::instantiate(&mut store, &component, &linker)?;
    let result = bindings.call_format_str(&mut store, "a", "b")?;
    println!("format_str: {}", result);
    Ok(())
}
