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

    let bindings = Formatter::instantiate(&mut store, &component, &linker)?;
    let result = bindings.call_format_str(&mut store, "a", "b")?;
    println!("format_str: {}", result);
    Ok(())
}
