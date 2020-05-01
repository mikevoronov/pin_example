use std::pin::Pin;
use wasmer_runtime::{compile, Func, ImportObject, Instance};

struct T<'a> {
    instance: Instance,
    func: Option<Func<'a, i32, i32>>,
}

impl T<'_> {
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
        let func = Some(res.as_ref().instance.exports.get::<Func<'_, i32, i32>>("ads").unwrap());
        let mut_ref = Pin::as_mut(&mut res);
        Pin::get_mut(mut_ref).func = func;
        //res.as_mut().func = func;

        res
    }
}

fn main() {
    let _t = T::new();
}
