use app::App;
use collection::get_collections;

pub mod app;
pub mod collection;
pub mod loader;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([1000.0, 1000.0]),
        ..Default::default()
    };

    let data = get_collections().expect("Failed fetching collections");

    eframe::run_native(
        "My egui App",
        options,
        App::creator(data),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let data = get_collections()
            .await
            .expect("Failed fetching collections");

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let canvas = document.get_element_by_id("the_canvas_id").expect("should have a canvas in document");

        eframe::WebRunner::new()
            .start(
                web_sys::HtmlCanvasElement::from(wasm_bindgen::JsValue::from(canvas)),
                web_options,
                App::creator(data),
            )
            .await
            .expect("failed to start eframe");
    });
}
