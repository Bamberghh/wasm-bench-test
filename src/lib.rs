use libloading::os::windows as libloading_imp;
use wasmtime::{
    Config, Engine, Func, Instance, Module, Store, TypedFunc,
    component::{self, Component},
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxView, WasiView};

pub struct CDylibHost {
    add: libloading_imp::Symbol<unsafe extern "C" fn(i32, i32) -> i32>,
    add_host_x:
        libloading_imp::Symbol<unsafe extern "C" fn(unsafe extern "C" fn(i32) -> i32, i32) -> i32>,
    _lib: libloading::Library,
}

impl CDylibHost {
    extern "C" fn add_host_y(x: i32) -> i32 {
        x + 1
    }

    pub fn new() -> Result<Self, libloading::Error> {
        unsafe {
            let lib = libloading::Library::new("cdylib/target/release/wasm_bench_test_cdylib.dll")?;
            let add: libloading::Symbol<unsafe extern "C" fn(i32, i32) -> i32> = lib.get(b"add")?;
            let add_host_x: libloading::Symbol<
                unsafe extern "C" fn(unsafe extern "C" fn(i32) -> i32, i32) -> i32,
            > = lib.get(b"add_host_x")?;
            Ok(Self {
                add: add.into_raw(),
                add_host_x: add_host_x.into_raw(),
                _lib: lib,
            })
        }
    }

    pub fn call_add(&self, x: i32, y: i32) -> i32 {
        unsafe { (self.add)(x, y) }
    }

    pub fn call_add_host(&self, x: i32) -> i32 {
        unsafe { (self.add_host_x)(Self::add_host_y, x) }
    }
}

pub struct ModuleState {}

pub struct ModuleHost {
    store: Store<ModuleState>,
    add: TypedFunc<(i32, i32), (i32,)>,
    add_host_x: TypedFunc<(i32,), (i32,)>,
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
        let add_host_y = Func::wrap(&mut store, |x: i32| x + 1);
        let instance = Instance::new(&mut store, &module, &[add_host_y.into()])?;
        let add = instance.get_typed_func(&mut store, "add")?;
        let add_host_x = instance.get_typed_func(&mut store, "add_host_x")?;
        Ok(Self {
            store,
            add,
            add_host_x,
        })
    }

    pub fn call_add(&mut self, x: i32, y: i32) -> wasmtime::Result<i32> {
        self.add.call(&mut self.store, (x, y)).map(|(r,)| r)
    }

    pub fn call_add_host(&mut self, x: i32) -> wasmtime::Result<i32> {
        self.add_host_x.call(&mut self.store, (x,)).map(|(r,)| r)
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
    add_host_x: component::TypedFunc<(i32,), (i32,)>,
}

impl ComponentHost {
    pub fn new() -> wasmtime::Result<Self> {
        let mut config = Config::new();
        config.strategy(wasmtime::Strategy::Cranelift);
        let engine = Engine::new(&config)?;
        let mut linker = component::Linker::new(&engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
        let mut host_interface = linker.instance("wasm-bench-test:component/host@0.1.0")?;
        host_interface.func_wrap("add-host-y", |_store, (x,): (i32,)| Ok((x + 1,)))?;
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
        let add_host_x_idx = instance
            .get_export_index(&mut store, parent_export_idx, "add-host-x")
            .expect("Cannot get `add-host-x` function");
        let add_host_x_untyped = instance
            .get_func(&mut store, add_host_x_idx)
            .expect("Unreachable since we've got add_host_x_idx");
        let add_host_x = add_host_x_untyped.typed(&mut store)?;
        Ok(Self {
            store,
            add,
            add_host_x,
        })
    }

    pub fn call_add(&mut self, x: i32, y: i32) -> wasmtime::Result<i32> {
        self.add.call(&mut self.store, (x, y)).map(|(r,)| r)
    }

    pub fn call_add_host(&mut self, x: i32) -> wasmtime::Result<i32> {
        self.add_host_x.call(&mut self.store, (x,)).map(|(r,)| r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cdylib() -> Result<(), libloading::Error> {
        let host = CDylibHost::new()?;
        assert_eq!(host.call_add(1, 2), 3);
        assert_eq!(host.call_add_host(1), 3);
        Ok(())
    }

    #[test]
    fn module() -> wasmtime::Result<()> {
        let mut host = ModuleHost::new()?;
        assert_eq!(host.call_add(1, 2)?, 3);
        assert_eq!(host.call_add_host(1)?, 3);
        Ok(())
    }

    #[test]
    fn component() -> wasmtime::Result<()> {
        let mut host = ComponentHost::new()?;
        assert_eq!(host.call_add(1, 2)?, 3);
        Ok(())
    }
}
