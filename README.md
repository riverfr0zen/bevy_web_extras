
# Running the examples

## match_window

```
cargo build --release --example match_window --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/match_window.wasm
python3 -m http.server
```

Now go to http://0.0.0.0:8000/www/match_window.html

