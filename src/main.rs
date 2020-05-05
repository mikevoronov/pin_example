use std::mem;
use std::ops::Deref;
use std::pin::Pin;
use wasmer_runtime::{compile, Func, ImportObject, Instance};

struct T {
    instance: Instance,
    func: Option<Func<'static, i32, i32>>,
}

impl T {
    pub fn new() -> Pin<Box<Self>> {
        let import_object = ImportObject::new();
        let wasm_bytes = vec![];
        let instance = compile(&wasm_bytes)
            .unwrap()
            .instantiate(&import_object)
            .unwrap();
        let mut res = Box::pin(Self {
            instance,
            func: None,
        });

        unsafe {
            let func = res
                .deref()
                .instance
                .exports
                .get::<Func<'_, i32, i32>>("asd")
                .unwrap();
            let static_func = mem::transmute::<Func<'_, i32, i32>, Func<'static, i32, i32>>(func);
            Pin::get_unchecked_mut(res.as_mut()).func = Some(static_func);
        }
        //res.as_mut().func = func;

        res
    }
}

fn main() {
    let _t = T::new();
}
