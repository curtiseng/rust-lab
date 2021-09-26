#[link(wasm_import_module = "utilities")]
extern "C" {
    pub fn random() -> i32;
}

#[no_mangle]
extern "C" fn addto(x: i32) -> i32 {
    unsafe { x + random() }
}
