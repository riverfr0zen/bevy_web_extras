# Intro/description/caveats

Provides some convenience functionality to make working with Bevy web targets easier.


# Features:

* [x] Handle matching canvas to browser window size on startup and resize
* [x] Match canvas to a percentage of browser window size (can set for width and height separately)
* [x] Workaround for scale factor issues causing [WASM targets to crash on some mobile devices when trying to match window size](https://github.com/bevyengine/bevy/discussions/4021)        
* [x] Provides a `BrowserResized` event that triggers when the browser window is resized
* [x] Match browser document background color to the app's ClearColor resource on app startup
* [x] Option to match ClearColor as above **on every resize check** instead of just on setup
* [x] Specify the target canvas element id (defaults to "window-matching-canvas") 



# Running the examples

(TODO: more examples soon)


## match_window

```
cargo build --release --example match_window --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/match_window.wasm
python3 -m http.server
```

Now go to http://0.0.0.0:8000/www/match_window.html

