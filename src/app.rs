use crate::collection::Collection;

use eframe::egui::{self, collapsing_header::CollapsingState};
use egui_extras::{Column, TableBuilder};

#[derive(Default)]
pub struct App {
    data: Collection,
    search: String,
    wantlist: String,
}

impl App {
    pub fn new(data: Collection) -> Self {
        Self {
            data,
            ..Default::default()
        }
    }

    fn mk_table(&self, ui: &mut egui::Ui) {
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
                    .filter(|data| {
                        if self.wantlist.is_empty() {
                            data.matches(&self.search)
                        } else {
                            self.wantlist.lines().any(|search| data.matches(search))
                        }
                    })
                    .collect();
                body.rows(20.0, data.len(), |mut row| {
                    let row_index = row.index();
                    data[row_index].as_row(&mut row);
                })
            });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Search: ");
                egui::TextEdit::singleline(&mut self.search)
                    .interactive(self.wantlist.is_empty())
                    .text_color_opt(
                        (!self.wantlist.is_empty()).then(|| egui::Color32::from_gray(64)),
                    )
                    .show(ui);
            });

            ui.collapsing("Advanced", |ui| {
                ui.label("Wantlist");
                egui::ScrollArea::vertical()
                    .max_height(150.0)
                    .show(ui, |ui| {
                        egui::TextEdit::multiline(&mut self.wantlist)
                            .desired_rows(10)
                            .show(ui);
                    });
            });

            ui.separator();

            self.mk_table(ui);
        });
    }
}
