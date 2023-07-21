use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::{ WasiCtxBuilder, WasiCtx, Table, WasiView };

bindgen!({
    async: true
});

struct MyState {
    table: Table,
    wasi: WasiCtx,
}

impl WasiView for MyState {
    fn ctx(&
        self) -> &WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
}

// Imports into the world, like the `name` import for this world, are satisfied
// through traits.
#[async_trait::async_trait]
impl Host_Imports for MyState {
    // Note the `Result` return value here where `Ok` is returned back to
    // the component and `Err` will raise a trap.
    async fn print(&mut self, msg: String) -> wasmtime::Result<()> {
        println!("hello from the host: {}", msg);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> wasmtime::Result<()> {
    let component_path = std::env::args().nth(1).expect("must provide path to component");

    // Configure an `Engine` and compile the `Component` that is being run for
    // the application.
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, component_path)?;
    let mut table = Table::new();
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .build(&mut table)
        .expect("failed to build WASI context");

    

    // Instantiation of bindings always happens through a `Linker`.
    // Configuration of the linker is done through a generated `add_to_linker`
    // method on the bindings structure.
    //
    // Note that the closure provided here is a projection from `T` in
    // `Store<T>` to `&mut U` where `U` implements the `HelloWorldImports`
    // trait. In this case the `T`, `MyState`, is stored directly in the
    // structure so no projection is necessary here.
    let mut linker = Linker::new(&engine);
    Host_::add_to_linker(&mut linker, |state: &mut MyState| state)?;
    
    wasmtime_wasi::preview2::wasi::command::add_to_linker(&mut linker)?;
    // As with the core wasm API of Wasmtime instantiation occurs within a
    // `Store`. The bindings structure contains an `instantiate` method which
    // takes the store, component, and linker. This returns the `bindings`
    // structure which is an instance of `HelloWorld` and supports typed access
    // to the exports of the component.
    let mut store = Store::new(
        &engine,
        MyState {
            table,
            wasi
        },
    );
    let (bindings, _) = Host_::instantiate_async(&mut store, &component, &linker).await?;

    // Here our `greet` function doesn't take any parameters for the component,
    // but in the Wasmtime embedding API the first argument is always a `Store`.
    bindings.call_run(&mut store).await?;
    Ok(())
}