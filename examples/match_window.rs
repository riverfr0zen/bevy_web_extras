use bevy::prelude::*;
use bevy_web_extras::prelude::*;


pub fn main() {
    let winsetup = WindowSetup::default();
    let mut app = web_app(winsetup);
    app.run();
}
