```shell
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

正如Rust参考所指出的那样：我们正在创建一个动态库,以从另一种语言加载.那么为什么输出都不是.dll,.so或.dylib？那是因为我们没有针对Windows,
Linux或MacOS进行编译.我们正在为wasm32-unknown-unknown进行编译.因此,此处参考文献的唯一缺点是未列出所有可能的平台及其动态库文件的结尾.
