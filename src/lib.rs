#[cfg(target_arch = "wasm32")]
use bevy::core::FixedTimestep;
use bevy::prelude::*;
#[cfg(target_arch = "wasm32")]
use bevy::render::renderer::RenderDevice;
#[cfg(target_arch = "wasm32")]
use bevy::window::WindowCreated;
#[cfg(target_arch = "wasm32")]
use web_sys::Window as WebsysWindow;

pub mod prelude;

const TARGET_RES_WIDTH: f32 = 3840.0;
// const TARGET_RES_HEIGHT: f32 = 2160.0;
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
// These *_DEV settings are based on nothing but my current display & prefs
// const WINDOW_WIDTH_DEV: f32 = 2400.0;
// const WINDOW_HEIGHT_DEV: f32 = 2000.0;
const WINDOW_WIDTH_DEV: f32 = 1400.0;
const WINDOW_HEIGHT_DEV: f32 = 1200.0;
#[cfg(target_arch = "wasm32")]
const RESIZE_CHECK_STEP: f64 = 1.0;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


/// This Resource is currently a little amorphous as it contains both config and some state details.
/// Probably want to separate this out later.
#[derive(Clone, Debug)]
pub struct WebExtrasCfg {
    pub title: String,
    /// The id of the canvas element we are targetting
    pub canvas: String,
    /// Multiplier of window width that canvas size should match. Defaults to 1.0 (100%).
    pub canvas_match_w: f32,
    /// Multiplier of window height that canvas size should match. Defaults to 1.0 (100%).
    pub canvas_match_h: f32,
    /// Alternatively match window dimensions to an element with this id
    pub match_element: Option<String>,
    /// Whether the HTML document background should match the app's ClearColor resource on app startup
    pub match_clear_color: bool,
    // Same as `match_clear_color`, but match on *every resize check*
    pub match_clear_color_always: bool,
    /// An optional initial width of the canvas. Set to WINDOW_WIDTH or WINDOW_WIDTH_DEV by default,
    /// depending on debug_assertions
    pub width: f32,
    /// Like `width` above
    pub height: f32,
    /// Convenience access to max_x
    pub max_x: f32,
    /// Convenience access to max_y
    pub max_y: f32,
    /// This is not actually related to web, but sets the window position of the native app
    /// against the TARGET_RES_WIDTH, which was useful for me while debugging. Will probably
    /// take this out later.
    pub position_x: f32,
    pub position_y: f32,
}


impl Default for WebExtrasCfg {
    fn default() -> Self {
        if cfg!(debug_assertions) {
            Self {
                title: String::from("Untitled Bevy web app"),
                canvas: String::from("#window-matching-canvas"),
                canvas_match_w: 1.0,
                canvas_match_h: 1.0,
                match_element: None,
                match_clear_color: false,
                match_clear_color_always: false,
                width: WINDOW_WIDTH_DEV,
                height: WINDOW_HEIGHT_DEV,
                max_x: WINDOW_WIDTH_DEV / 2.0,
                max_y: WINDOW_HEIGHT_DEV / 2.0,
                // Align window to the right of screen
                // position_x: TARGET_RES_WIDTH - WINDOW_WIDTH_DEV,
                position_x: 0.0,
                position_y: 0.0,
            }
        } else {
            Self {
                title: String::from("Untitled Bevy web app"),
                canvas: String::from("#window-matching-canvas"),
                canvas_match_w: 1.0,
                canvas_match_h: 1.0,
                match_element: None,
                match_clear_color: false,
                match_clear_color_always: false,
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                max_x: WINDOW_WIDTH / 2.0,
                max_y: WINDOW_HEIGHT / 2.0,
                position_x: TARGET_RES_WIDTH - WINDOW_WIDTH,
                position_y: 0.0,
            }
        }
    }
}


#[cfg(target_arch = "wasm32")]
pub struct BrowserResized;


/// Match html body background to clear color
#[cfg(target_arch = "wasm32")]
fn match_clear_color(wasm_window: &WebsysWindow, app_clear_color: Color) {
    let body = wasm_window.document().unwrap().body().unwrap();
    let _ = body.style().set_property(
        "background-color",
        format!(
            "rgb({}, {}, {})",
            app_clear_color.r() * 255.0,
            app_clear_color.g() * 255.0,
            app_clear_color.b() * 255.0
        )
        .as_str(),
    );
}


/// Based on https://github.com/bevyengine/bevy/issues/175
///
/// Call the handle_browser_resize system once at startup (if window is created)
/// to cover for the short period before handle_browser_resize kicks in
/// (since that system will likely be set to a FixedTimeStep)
#[cfg(target_arch = "wasm32")]
fn setup_browser(
    app_clear_color: Res<ClearColor>,
    webcfg: ResMut<WebExtrasCfg>,
    windows: ResMut<Windows>,
    resize_event_writer: EventWriter<BrowserResized>,
    render_device: Res<RenderDevice>,
    mut window_created_reader: EventReader<WindowCreated>,
) {
    if window_created_reader.iter().next().is_some() {
        let wasm_window = web_sys::window().unwrap();

        // Set title
        wasm_window.document().unwrap().set_title(&webcfg.title);

        // Match html body background to clear color
        if webcfg.match_clear_color {
            match_clear_color(&wasm_window, app_clear_color.0);
        }
        handle_browser_resize(
            app_clear_color,
            render_device,
            webcfg,
            windows,
            resize_event_writer,
        );
    }
}


/// Based on this Discord conversation: https://i.imgur.com/osfA8PH.png AND
/// https://github.com/mrk-its/bevy-robbo/blob/master/src/main.rs
#[cfg(target_arch = "wasm32")]
fn handle_browser_resize(
    app_clear_color: Res<ClearColor>,
    render_device: Res<RenderDevice>,
    mut webcfg: ResMut<WebExtrasCfg>,
    mut windows: ResMut<Windows>,
    mut resize_event_writer: EventWriter<BrowserResized>,
) {
    let window = windows.get_primary_mut().unwrap();
    let wasm_window = web_sys::window().unwrap();

    // Match html body background to clear color on every resize
    if webcfg.match_clear_color_always {
        match_clear_color(&wasm_window, app_clear_color.0);
    }

    let mut target_width: f32;
    let mut target_height: f32;
    if let Some(element_id) = &webcfg.match_element {
        let el_dom_rect = wasm_window
            .document()
            .unwrap()
            .get_element_by_id(element_id)
            .unwrap()
            .get_bounding_client_rect();
        // @TODO: consider modifying dimensions by `webcfg.canvas_match_w` and `webcfg.canvas_match_w`
        // as done below for matching whole window
        (target_width, target_height) = (el_dom_rect.width() as f32, el_dom_rect.height() as f32);
    } else {
        (target_width, target_height) = (
            wasm_window.inner_width().unwrap().as_f64().unwrap() as f32 * webcfg.canvas_match_w,
            wasm_window.inner_height().unwrap().as_f64().unwrap() as f32 * webcfg.canvas_match_h,
        );
    }

    // debug!("wasm_window.device_pixel_ratio: {}", wasm_window.device_pixel_ratio());
    // debug!("window.scale_factor: {}", window.scale_factor());
    // debug!("window.backend_scale_factor: {}", window.backend_scale_factor());
    // debug!("window.width: {}", window.width());
    // debug!("window.height: {}", window.height());
    // debug!("window.physical_width: {}", window.physical_width());
    // debug!("window.physical_height: {}", window.physical_height());
    // debug!("target_width: {}", target_width);
    // debug!("target_height: {}", target_height);


    // Handle scale factor differences resulting in a texture memory error on
    // some mobile devices
    // See: https://github.com/bevyengine/bevy/discussions/4021
    if window.scale_factor() >= 1.0 {
        let max_2d = render_device.limits().max_texture_dimension_2d;
        let scale_factor = window.scale_factor() as f32;

        if target_width * scale_factor > max_2d as f32 {
            target_width = (max_2d as f32 / scale_factor).floor();
            // debug!("corrected target_width: {}", target_width);
        }
        if target_height * scale_factor > max_2d as f32 {
            target_height = (max_2d as f32 / scale_factor).floor();
            // debug!("corrected target_height: {}", target_height);
        }
    }

    // Because there can be variations in floating point values between window dimensions
    // and target dimensions, we are using floor here to detect changes. Otherwise, this was
    // triggering on every step.
    //
    // if window.width() != target_width || window.height() != target_height {
    if window.width().floor() != target_width.floor()
        || window.height().floor() != target_height.floor()
    {
        // debug!(
        //     "{:?} {:?}, {:?} {:?}",
        //     window.width(),
        //     target_width,
        //     window.height(),
        //     target_height,
        // );
        window.set_resolution(target_width, target_height);
        webcfg.width = target_width;
        webcfg.height = target_height;
        webcfg.max_x = webcfg.width / 2.0;
        webcfg.max_y = webcfg.height / 2.0;
        resize_event_writer.send(BrowserResized)
    }
}


/// Create a web-oriented Bevy app that matches canvas to window size
pub fn web_app(webcfg: WebExtrasCfg) -> App {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: webcfg.title.to_string(),
        width: webcfg.width,
        height: webcfg.height,
        position: Some(Vec2::new(webcfg.position_x, webcfg.position_y)),
        #[cfg(target_arch = "wasm32")]
        canvas: Some(webcfg.canvas.to_string()),
        ..Default::default()
    })
    .insert_resource(webcfg);

    #[cfg(target_arch = "wasm32")]
    app.add_event::<BrowserResized>();

    #[cfg(target_arch = "wasm32")]
    app.add_startup_system(setup_browser);

    #[cfg(target_arch = "wasm32")]
    app.add_system(handle_browser_resize.with_run_criteria(FixedTimestep::step(RESIZE_CHECK_STEP)));


    return app;
}
