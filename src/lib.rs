use wasmtime::{
    Config, Engine, Instance, Module, Store, TypedFunc,
    component::{self, Component},
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxView, WasiView};

pub struct ModuleState {}

pub struct ModuleHost {
    store: Store<ModuleState>,
    add: TypedFunc<(i32, i32), (i32,)>,
}

impl ModuleHost {
    pub fn new() -> wasmtime::Result<Self> {
        let mut config = Config::new();
        config.strategy(wasmtime::Strategy::Cranelift);
        let engine = Engine::new(&config)?;
        let module_path =
            "module/target/wasm32-unknown-unknown/release/wasm_bench_test_module.wasm";
        let module = Module::from_file(&engine, module_path)?;
        let mut store = Store::new(&engine, ModuleState {});
        let instance = Instance::new(&mut store, &module, &[])?;
        let add = instance.get_typed_func(&mut store, "add")?;
        Ok(Self { store, add })
    }

    pub fn call_add(&mut self, x: i32, y: i32) -> wasmtime::Result<i32> {
        self.add.call(&mut self.store, (x, y)).map(|(r,)| r)
    }
}

pub struct ComponentState {
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
}
impl WasiView for ComponentState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi_ctx,
            table: &mut self.resource_table,
        }
    }
}

pub struct ComponentHost {
    store: Store<ComponentState>,
    add: component::TypedFunc<(i32, i32), (i32,)>,
}

impl ComponentHost {
    pub fn new() -> wasmtime::Result<Self> {
        let mut config = Config::new();
        config.strategy(wasmtime::Strategy::Cranelift);
        let engine = Engine::new(&config)?;
        let mut linker = component::Linker::new(&engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
        let state = ComponentState {
            wasi_ctx: WasiCtx::builder().build(),
            resource_table: ResourceTable::new(),
        };
        let mut store = Store::new(&engine, state);
        let component_path =
            "component/target/wasm32-wasip2/release/wasm_bench_test_component.wasm";
        let component = Component::from_file(&engine, component_path)?;
        let instance = linker.instantiate(&mut store, &component)?;
        let interface_idx = instance
            .get_export_index(
                &mut store,
                None,
                "wasm-bench-test:component/component@0.1.0",
            )
            .expect("Cannot get `wasm-bench-test:component/component@0.1.0` interface");
        let parent_export_idx = Some(&interface_idx);
        let add_idx = instance
            .get_export_index(&mut store, parent_export_idx, "add")
            .expect("Cannot get `add` function");
        let add_untyped = instance
            .get_func(&mut store, add_idx)
            .expect("Unreachable since we've got add_idx");
        let add = add_untyped.typed(&mut store)?;
        Ok(Self { store, add })
    }

    pub fn call_add(&mut self, x: i32, y: i32) -> wasmtime::Result<i32> {
        self.add.call(&mut self.store, (x, y)).map(|(r,)| r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module() -> wasmtime::Result<()> {
        let mut host = ModuleHost::new()?;
        let result = host.call_add(1, 2)?;
        assert_eq!(result, 3);
        Ok(())
    }

    #[test]
    fn component() -> wasmtime::Result<()> {
        let mut host = ComponentHost::new()?;
        let result = host.call_add(1, 2)?;
        assert_eq!(result, 3);
        Ok(())
    }
}
