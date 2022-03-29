# Intro/description/caveats

[TODO]


# Features:

* [x] Handle matching canvas to browser window size
* [x] Workaround for scale factor issues causing [WASM targets to crash on some mobile devices when trying to match window size](See https://doc.rust-lang.org/cargo/reference/features.html)        
* [x] Provides a `BrowserResized` event that triggers when the browser window is resized
* [x] Set browser document background color (e.g. to help match the Bevy app's ClearColor) on startup
* [x] Set browser document background color to match the app's ClearColor resource on app setup
* [x] Option to match ClearColor As above, except  **on every resize check** instead of just on setup
* [x] Specify canvas element id (currently hardcoded to "window-matching-canvas")
* [x] Ability to match canvas to a percentage of browser window size (can set for width and height separately)



# Running the examples

## match_window

```
cargo build --release --example match_window --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/match_window.wasm
python3 -m http.server
```

Now go to http://0.0.0.0:8000/www/match_window.html

