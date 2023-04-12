- wasmへのビルド
```
$ cargo b --target wasm32-unknown-unknown
$ wasm-pack build --target web
```

- [WIP] Rustでの実行
```
$ cargo r --example piston 
```

- Todo
	- Rust単体でもexampleから実行できるようにする
