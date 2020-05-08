use std::mem;
use std::ops::Deref;
use wasmer_runtime::{compile, Func, ImportObject, Instance};

struct T {
    instance: Instance,
    func: Func<'static, i32, i32>,
}

impl T {
    pub fn new() -> Self {
        let import_object = ImportObject::new();
        let wasm_bytes = vec![];
        let instance = compile(&wasm_bytes)
            .unwrap()
            .instantiate(&import_object)
            .unwrap();

        unsafe {
            let func = instance.exports.get::<Func<'_, i32, i32>>("asd").unwrap();
            let func = mem::transmute::<Func<'_, i32, i32>, Func<'static, i32, i32>>(func);
            Self {
                instance,
                func
            }
        }
    }
}

fn main() {
    let _t = T::new();
}
