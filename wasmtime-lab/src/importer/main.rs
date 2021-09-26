use std::io::Read;
use std::{error::Error, fs::File};

use anyhow::Result;
use wasmtime::*;

fn main() -> Result<(), Box<dyn Error>> {
    let bytes = {
        let mut f = File::open("wasmtime-lab/importer.wasm")?;

        let mut bytes = vec![];
        f.read_to_end(&mut bytes)?;
        bytes
    };

    println!("Initializing...");
    let store = Store::default();

    // Compile the wasm binary into an in-memory instance of a `Module`.
    println!("Compiling module...");
    let module = Module::from_file(store.engine(), bytes.as_ref())?;

    if let Err(_e) = module.link_closure(
        "utilities",
        "random",
        move |_ctx: &CallContext, ()| -> i32 {
            use rand::Rng;
            let mut rng = rand::thread_rng();

            rng.gen_range(0..=100)
        },
    ) {
        return Err("Failed to link closure".into());
    }

    let addto = module.find_function::<i32, i32>("addto")?;

    println!("Result: {}", addto.call(10)?);

    Ok(())
}
