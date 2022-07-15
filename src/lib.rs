use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;


// change these imports to change how things will be rendered (don't forget to also change Cargo.toml)
//const rcid: &str = "webgl2"; mod renderer_webgl; use renderer_webgl as renderer; use web_sys::WebGl2RenderingContext as RenderingContext; mod shader_program; mod cube;
const RCID: &str = "2d"; mod renderer_2d; use renderer_2d as renderer; use web_sys::CanvasRenderingContext2d as RenderingContext;
//

mod world;
mod assets;
mod interactions;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    //  <MARK>

    let canvas: HtmlCanvasElement = document.get_element_by_id("wasm").unwrap().dyn_into().unwrap();

    document.set_title("Loading...");

    let mut pixel_width_f: f64 = 0.0;
    let mut pixel_height_f: f64 = 0.0;
    let mut pixel_width_i: u32 = 0;
    let mut pixel_height_i: u32 = 0;
    let mut update_canvas_size = || {
        pixel_width_f = window.inner_width().unwrap().as_f64().unwrap();
        pixel_height_f = window.inner_height().unwrap().as_f64().unwrap();
        pixel_width_i = pixel_width_f.round() as u32; pixel_height_i = pixel_height_f.round() as u32;
        canvas.set_width(pixel_width_i);
        canvas.set_height(pixel_height_i);
    };
    update_canvas_size();

    document.set_title("Started.");

    // </MARK>

    if let Some(context) = canvas.get_context(RCID)? {
        let context: RenderingContext = context.dyn_into().unwrap();
        // context.clear_color(0.0, 0.0, 0.0, 1.0);
        // context.clear(RenderingContext::COLOR_BUFFER_BIT);
        renderer::init_renderer(context, pixel_width_i, pixel_height_i, window, document);
    }

    Ok(())
}
