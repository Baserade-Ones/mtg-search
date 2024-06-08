use archidekt::User;
use eframe::egui;
use std::io::Cursor;
use strum::VariantArray;

pub type Collection = Vec<Entry>;
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Entry {
    #[serde(skip)]
    owner: Option<User>,
    #[serde(rename = "Quantity")]
    quantity: u8,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Edition Code")]
    set: String,
    #[serde(rename = "Multiverse Id")]
    multiverse: String,
    #[serde(rename = "Scryfall ID")]
    scryfall: String,
    #[serde(rename = "Price (Card Market)")]
    price: f32,
}

impl Entry {
    pub fn as_row(&self, row: &mut egui_extras::TableRow) {
        let owner = self.owner.unwrap();
        row.col(|ui| {
            ui.colored_label(color_code_user(owner), owner.to_string());
        });
        row.col(|ui| {
            ui.label(self.quantity.to_string());
        });
        row.col(|ui| {
            ui.label(self.name.clone());
        });
        row.col(|ui| {
            ui.label(self.set.clone());
        });
        row.col(|ui| {
            ui.label(self.multiverse.clone());
        });
        row.col(|ui| {
            ui.label(self.scryfall.clone());
        });
        row.col(|ui| {
            ui.label(format!("{:.2}€", self.price));
        });
    }

    pub fn matches(&self, search: String) -> bool {
        //TODO search syntax
        self.name.to_lowercase().contains(&search.to_lowercase())
    }
}

fn color_code_user(value: User) -> egui::Color32 {
    match value {
        User::Strosel => egui::Color32::from_rgb(0xbe, 0x18, 0x5d),
        User::Amon8808 => egui::Color32::from_rgb(0x03, 0x69, 0xa1),
        //TODO User::MathIsMath => egui::Color32::from_rgb(0x0f, 0x76, 0x6e),
        User::Urgalurga => egui::Color32::from_rgb(0xc2, 0x41, 0x0c),
        User::TheColdPanda => egui::Color32::from_rgb(0xb9, 0x1c, 0x1c),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_collections() -> anyhow::Result<Collection> {
    let mut collections = Collection::with_capacity(1000 * User::VARIANTS.len());

    for user in User::VARIANTS {
        let data = archidekt::get_collections(user)?;

        let mut reader = csv::Reader::from_reader(Cursor::new(data));

        let mut col = reader
            .deserialize::<Entry>()
            .map(|ent| {
                let mut ent = ent?;
                ent.owner = Some(*user);
                Ok(ent)
            })
            .collect::<Result<Vec<Entry>, csv::Error>>()?;

        collections.append(&mut col);
    }

    Ok(collections)
}

#[cfg(target_arch = "wasm32")]
pub async fn get_collections() -> anyhow::Result<Collection> {
    let mut collections = Collection::with_capacity(1000 * User::VARIANTS.len());

    for user in User::VARIANTS {
        let req = ehttp::Request::get(format!("assets/{user}.csv"));
        let resp = ehttp::fetch_async(req).await.map_err(anyhow::Error::msg)?;
        let data = resp
            .text()
            .ok_or_else(|| anyhow::Error::msg("Empty CSV body"))?;

        let mut reader = csv::Reader::from_reader(Cursor::new(data));

        let mut col = reader
            .deserialize::<Entry>()
            .map(|ent| {
                let mut ent = ent?;
                ent.owner = Some(*user);
                Ok(ent)
            })
            .collect::<Result<Vec<Entry>, csv::Error>>()?;

        collections.append(&mut col);
    }

    Ok(collections)
}
