use crate::collection::*;
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
        User::VikinGG => Color32::from_rgb(0xc1, 0x54, 0xc1),
        //User::Elin => Color32::from_rgb(0xc0, 0x34 ,0xeb),
        User::OliverDizz => Color32::from_rgb(0x00, 0x61, 0x0d),
    }
}

fn color_ident(color: char) -> egui::RichText {
    match color {
        'W' => egui::RichText::new("\u{e600}").color(Color32::from_rgb(251, 246, 211)),
        'U' => egui::RichText::new("\u{e601}").color(Color32::from_rgb(150, 196, 212)),
        'B' => egui::RichText::new("\u{e602}").color(Color32::from_rgb(176, 168, 163)),
        'R' => egui::RichText::new("\u{e603}").color(Color32::from_rgb(216, 143, 115)),
        'G' => egui::RichText::new("\u{e604}").color(Color32::from_rgb(147, 203, 164)),
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
    dedup: bool,
}

impl App {
    pub fn new(data: Collection) -> Self {
        Self {
            data,
            search: Search::single(),
            dedup: false,
        }
    }

    pub fn creator(data: Collection) -> eframe::AppCreator<'static> {
        use super::loader;
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            cc.egui_ctx
                .add_image_loader(std::sync::Arc::new(loader::Image::new()));

            loader::load_fonts(&cc.egui_ctx);

            Ok(Box::new(App::new(data)))
        })
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
                    .dedup_cards(self.dedup)
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
                ui.separator();
                ui.checkbox(&mut self.dedup, "Group printings?");
            });

            match self.search {
                Search::Single {
                    ref mut owner,
                    ref mut color,
                    ref mut colorless,
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
                            egui::ComboBox::from_id_salt("owner")
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
                                    color[i] = !color[i];
                                    *colorless = false;
                                }
                            }

                            if ui
                                .add(egui::SelectableLabel::new(*colorless, color_ident('C')))
                                .clicked()
                            {
                                *colorless = !*colorless;
                                **color = [false; 5];
                            }
                        });

                        ui.spacing();

                        ui.horizontal(|ui| {
                            ui.label("Type: ");
                            egui::TextEdit::singleline(ty).show(ui);
                        });
                    });
                }
                Search::Wantlist(ref mut list, ref mut owner) => {
                    ui.label("Wantlist");
                    egui::ScrollArea::vertical()
                        .id_salt("wantlist")
                        .max_height(150.0)
                        .show(ui, |ui| {
                            egui::TextEdit::multiline(list).desired_rows(10).show(ui);
                        });

                    ui.spacing();

                    ui.horizontal(|ui| {
                        ui.label("Owner:");
                        egui::ComboBox::from_id_salt("owner")
                            .selected_text(owner.map_or(String::new(), |owner| owner.to_string()))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(owner, None, "");
                                for user in archidekt::User::VARIANTS {
                                    ui.selectable_value(owner, Some(*user), user.to_string());
                                }
                            })
                    });
                }
            }

            ui.separator();

            self.mk_table(ui);
        });
    }
}
