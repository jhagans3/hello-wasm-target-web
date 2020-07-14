source:
https://blog.logrocket.com/getting-started-with-webassembly-and-rust/
and
https://github.com/dmilford/rust-3d-demo

### 🛠️ Build with `wasm-pack build`

```
clear && wasm-pack build --target web && python3 -m http.server 8000
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```
