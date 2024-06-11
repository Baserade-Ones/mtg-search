use crate::collection::Search;
use archidekt::{Collection, Entry, User};
use strum::VariantArray;

use eframe::egui::{self, Color32};
use egui_extras::{Column, TableBuilder};

fn color_code_user(value: User) -> egui::Color32 {
    match value {
        User::Strosel => Color32::from_rgb(0xbe, 0x18, 0x5d),
        User::Amon8808 => Color32::from_rgb(0x03, 0x69, 0xa1),
        User::MathIsMath => Color32::from_rgb(0x0f, 0x76, 0x6e),
        User::Urgalurga => Color32::from_rgb(0xc2, 0x41, 0x0c),
        User::TheColdPanda => Color32::from_rgb(0xb9, 0x1c, 0x1c),
    }
}

fn color_ident(color: char) -> egui::RichText {
    match color {
        'W' => egui::RichText::new("\u{e600}").color(Color32::from_rgb(248, 231, 185)),
        'U' => egui::RichText::new("\u{e601}").color(Color32::from_rgb(179, 206, 234)),
        'B' => egui::RichText::new("\u{e602}").color(Color32::from_rgb(166, 159, 157)),
        'R' => egui::RichText::new("\u{e603}").color(Color32::from_rgb(235, 159, 130)),
        'G' => egui::RichText::new("\u{e604}").color(Color32::from_rgb(196, 211, 202)),
        _ => egui::RichText::new("\u{e904}"),
    }
    .font(egui::FontId {
        size: 14.0,
        family: egui::FontFamily::Name("Mana".into()),
    })
}

pub struct App {
    data: Collection,
    search: Search,
}

impl App {
    pub fn new(data: Collection) -> Self {
        Self {
            data,
            search: Search::single(),
        }
    }

    fn mk_table(&self, ui: &mut egui::Ui) {
        let table = TableBuilder::new(ui)
            .resizable(false)
            .striped(true)
            .column(Column::exact(90.0))
            .column(Column::exact(20.0))
            .column(Column::exact(100.0))
            .column(Column::exact(200.0))
            .column(Column::exact(200.0))
            .column(Column::exact(50.0))
            .column(Column::exact(250.0))
            .column(Column::exact(70.0));

        table
            .header(20.0, |mut header| {
                for hdr in Entry::headers() {
                    header.col(|ui| {
                        ui.strong(hdr);
                    });
                }
            })
            .body(|body| {
                let data: Collection = self
                    .data
                    .iter()
                    .filter(|&data| self.search.apply(data))
                    .cloned()
                    .collect();
                body.rows(20.0, data.len(), |mut row| {
                    let row_index = row.index();

                    for (header, field) in data[row_index].values() {
                        row.col(|ui| match header {
                            "Owner" => {
                                ui.colored_label(color_code_user(data[row_index].owner), field);
                            }
                            "Color Id" => {
                                ui.horizontal(|ui| {
                                    ui.style_mut().spacing.item_spacing = egui::vec2(1.0, 1.0);
                                    for c in field.chars() {
                                        ui.label(color_ident(c));
                                    }
                                });
                            }
                            "Scryfall" => {
                                ui.hyperlink_to(
                                    field.clone(),
                                    format!(
                                        "https://scryfall.com/search?q=scryfall_id%3A{}",
                                        field.clone()
                                    ),
                                )
                                .on_hover_ui(|ui| {
                                    ui.add(
                                        egui::Image::new(format!(
                                            "https://cards.scryfall.io/png/front/{}/{}/{}.png",
                                            field.as_bytes()[0] as char,
                                            field.as_bytes()[1] as char,
                                            field.clone()
                                        ))
                                        .fit_to_exact_size(egui::vec2(292.0, 408.0)),
                                    );
                                });
                            }
                            _ => {
                                ui.label(field);
                            }
                        });
                    }
                })
            });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.search, Search::single(), "Simple");
                ui.selectable_value(&mut self.search, Search::wantlist(), "Wantlist");
            });

            match self.search {
                Search::Single {
                    ref mut owner,
                    ref mut color,
                    ref mut name,
                    ref mut ty,
                } => {
                    ui.horizontal(|ui| {
                        ui.label("Search: ");
                        egui::TextEdit::singleline(name).show(ui);
                    });

                    ui.collapsing("Advanced", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Owner:");
                            egui::ComboBox::from_id_source("owner")
                                .selected_text(
                                    owner.map_or(String::new(), |owner| owner.to_string()),
                                )
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(owner, None, "");
                                    for user in archidekt::User::VARIANTS {
                                        ui.selectable_value(owner, Some(*user), user.to_string());
                                    }
                                })
                        });

                        ui.spacing();

                        ui.horizontal(|ui| {
                            ui.label("Color Identity:");
                            for (i, c) in "WUBRG".chars().enumerate() {
                                if ui
                                    .add(egui::SelectableLabel::new(color[i], color_ident(c)))
                                    .clicked()
                                {
                                    color[i] = !color[i]
                                }
                            }
                        });

                        ui.spacing();

                        ui.horizontal(|ui| {
                            ui.label("Type: ");
                            egui::TextEdit::singleline(ty).show(ui);
                        });
                    });
                }
                Search::Wantlist(ref mut wantlist) => {
                    ui.label("Wantlist");
                    egui::ScrollArea::vertical()
                        .id_source("wantlist")
                        .max_height(150.0)
                        .show(ui, |ui| {
                            egui::TextEdit::multiline(wantlist)
                                .desired_rows(10)
                                .show(ui);
                        });
                }
            }

            ui.separator();

            self.mk_table(ui);
        });
    }
}
