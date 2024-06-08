use collection::{get_collections, Collection};

use eframe::egui;
use egui_extras::{Column, TableBuilder};

mod collection;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 400.0]),
        ..Default::default()
    };

    let data = get_collections().expect("Failed fetching collections");

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

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
                Box::new(|_cc| Box::new(App::new(data))),
            )
            .await
            .expect("failed to start eframe");
    });
}

#[derive(Default)]
struct App {
    data: Collection,
    search: String,
}

impl App {
    fn new(data: Collection) -> Self {
        Self {
            data,
            search: String::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let name_label = ui.label("Search: ");
                ui.text_edit_singleline(&mut self.search)
                    .labelled_by(name_label.id);
            });

            let table = TableBuilder::new(ui)
                .resizable(false)
                .striped(true)
                .column(Column::exact(90.0))
                .column(Column::exact(20.0))
                .column(Column::exact(200.0))
                .column(Column::exact(50.0))
                .column(Column::exact(70.0))
                .column(Column::exact(250.0))
                .column(Column::exact(70.0));

            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("Owner");
                    });
                    header.col(|ui| {
                        ui.strong("X");
                    });
                    header.col(|ui| {
                        ui.strong("Name");
                    });
                    header.col(|ui| {
                        ui.strong("Set");
                    });
                    header.col(|ui| {
                        ui.strong("Multiverse");
                    });
                    header.col(|ui| {
                        ui.strong("Scryfall");
                    });
                    header.col(|ui| {
                        ui.strong("Price");
                    });
                })
                .body(|body| {
                    let data: Collection = self
                        .data
                        .iter()
                        .cloned()
                        .filter(|data| data.matches(self.search.clone()))
                        .collect();
                    body.rows(20.0, data.len(), |mut row| {
                        let row_index = row.index();
                        data[row_index].as_row(&mut row);
                    })
                });
        });
    }
}
