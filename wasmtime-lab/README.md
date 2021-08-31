**install wasmtime**
```bash
curl https://wasmtime.dev/install.sh -sSf | bash
```
**install rustc target**
```bash
rustup target add wasm32-wasi
```
**run wasm**
```bash
cargo build --target wasm32-wasi
wasmtime ../target/wasm32-wasi/debug/edge-hub.wasm
```
_or_
```bash
cargo install cargo-wasi
cargo wasi run
```