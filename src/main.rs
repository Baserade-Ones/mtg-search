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
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            cc.egui_ctx
                .add_image_loader(std::sync::Arc::new(loader::Image::new()));

            loader::load_fonts(&cc.egui_ctx);

            Box::new(App::new(data))
        }),
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
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| {
                    // This gives us image support:
                    egui_extras::install_image_loaders(&cc.egui_ctx);

                    cc.egui_ctx
                        .add_image_loader(std::sync::Arc::new(loader::Image::new()));

                    loader::load_fonts(&cc.egui_ctx);

                    Box::new(App::new(data))
                }),
            )
            .await
            .expect("failed to start eframe");
    });
}
