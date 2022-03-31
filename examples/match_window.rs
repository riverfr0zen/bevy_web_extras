use bevy_web_extras::prelude::*;


pub fn main() {
    let webcfg = WebExtrasCfg::default();
    let mut app = web_app(webcfg);
    app.run();
}
