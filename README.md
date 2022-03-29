# What it is

Bevy makes it quite easy to target a web release and support is growing steadily. However, as my work is going to be pretty web-focused I needed some additional baseline functionality for my projects.

Since implementing these took some digging and discussion for me as a newcomer, I figured others may benefit by releasing this repo.

Please note that I am new to both Rust and Bevy so any feedback (and pull requests!) will be appreciated. 


# Features:

* [x] Handle matching canvas to browser window size on startup and resize
* [x] Match canvas to a percentage of browser window size (can set for width and height separately)
* [x] Workaround for scale factor issues causing [WASM targets to crash on some mobile devices when trying to match window size](https://github.com/bevyengine/bevy/discussions/4021)        
* [x] Provides a `BrowserResized` event that triggers when the browser window is resized
* [x] Match browser document background color to the app's ClearColor resource on app startup
* [x] Option to match ClearColor as above **on every resize check** instead of just on setup
* [x] Specify the target canvas element id (defaults to "window-matching-canvas") 
* [x] Convenience `web_app()` function that wires up functionality in this crate into a "baseline web" app


# Synopsis

```rust

use bevy::prelude::*;
use bevy_web_extras::prelude::*;

pub fn main() {
    // ... Create an app with "baseline web functionality" ...
    let winsetup = WindowSetup {
        title: String::from("my example"),
        canvas: String::from("#window-matching-canvas"),
        /// Multiplier of browser window width that canvas size should match. Defaults to 1.0 (100%).
        canvas_match_w: 1.0,
        /// Multiplier of browser window height that canvas size should match. Defaults to 1.0 (100%).
        canvas_match_h: 1.0,
        /// Whether the HTML document background should match the app's ClearColor resource on app startup
        match_clear_color: false,
        // Same as `match_clear_color`, but match on *every resize check*
        // match_clear_color_always: false,
        ..Default::default()
    };
    let mut app = web_app(winsetup);

    app.insert_resource(ClearColor(CHANGER_CLEAR_CLR))
        .add_plugin(ShapePlugin)
        // ... BUILD APP AS USUAL ...
        .run();
}
```


# Running the examples

(TODO: more examples soon)


## match_window

```
cargo build --release --example match_window --target wasm32-unknown-unknown
wasm-bindgen --out-dir www/wasms --target web target/wasm32-unknown-unknown/release/examples/match_window.wasm
python3 -m http.server
```

Now go to http://0.0.0.0:8000/www/match_window.html

