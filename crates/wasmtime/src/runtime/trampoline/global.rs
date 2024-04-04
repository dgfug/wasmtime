use crate::store::{AutoAssertNoGc, StoreOpaque};
use crate::{GlobalType, Mutability, Result, RootedGcRefImpl, Val};
use std::ptr;
use wasmtime_runtime::{StoreBox, VMGlobalDefinition};

#[repr(C)]
pub struct VMHostGlobalContext {
    pub(crate) ty: GlobalType,
    pub(crate) global: VMGlobalDefinition,
}

pub fn generate_global_export(
    store: &mut StoreOpaque,
    ty: GlobalType,
    val: Val,
) -> Result<wasmtime_runtime::ExportGlobal> {
    let global = wasmtime_environ::Global {
        wasm_ty: ty.content().to_wasm_type(),
        mutability: match ty.mutability() {
            Mutability::Const => false,
            Mutability::Var => true,
        },
    };
    let ctx = StoreBox::new(VMHostGlobalContext {
        ty,
        global: VMGlobalDefinition::new(),
    });

    let mut store = AutoAssertNoGc::new(store);
    let definition = unsafe {
        let global = &mut (*ctx.get()).global;
        match val {
            Val::I32(x) => *global.as_i32_mut() = x,
            Val::I64(x) => *global.as_i64_mut() = x,
            Val::F32(x) => *global.as_f32_bits_mut() = x,
            Val::F64(x) => *global.as_f64_bits_mut() = x,
            Val::V128(x) => *global.as_u128_mut() = x.into(),
            Val::FuncRef(f) => {
                *global.as_func_ref_mut() =
                    f.map_or(ptr::null_mut(), |f| f.vm_func_ref(&mut store).as_ptr());
            }
            Val::ExternRef(x) => {
                let new = match x {
                    None => None,
                    Some(x) => Some(x.try_gc_ref(&mut store)?.unchecked_copy()),
                };
                let new = new.as_ref();
                global.write_gc_ref(store.gc_store_mut()?, new);
            }
            Val::AnyRef(a) => {
                let new = match a {
                    None => None,
                    Some(a) => Some(a.try_gc_ref(&mut store)?.unchecked_copy()),
                };
                let new = new.as_ref();
                global.write_gc_ref(store.gc_store_mut()?, new);
            }
        }
        global
    };

    store.host_globals().push(ctx);
    Ok(wasmtime_runtime::ExportGlobal {
        definition,
        vmctx: std::ptr::null_mut(),
        global,
    })
}
