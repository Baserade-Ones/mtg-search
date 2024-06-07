use eframe::egui;
use reqwest::blocking::Client;
use std::io::Cursor;
use strum::{Display, VariantArray};

#[derive(Debug, Clone, serde::Deserialize)]
struct Response {
    content: String,
    #[serde(rename = "totalRows")]
    rows: u16,
    #[serde(rename = "moreContent")]
    more: bool,
}

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
            ui.colored_label(owner, owner.to_string());
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VariantArray, Display)]
pub enum User {
    Strosel,
    Amon8808,
    MathIsMath,
    Urgalurga,
    TheColdPanda,
}

impl User {
    fn id(&self) -> u32 {
        match self {
            Self::Strosel => 331139,
            Self::Amon8808 => 324351,
            Self::MathIsMath => 331139, //TODO
            Self::Urgalurga => 382090,
            Self::TheColdPanda => 418756,
        }
    }
}

impl From<User> for egui::Color32 {
    fn from(value: User) -> Self {
        match value {
            User::Strosel => egui::Color32::from_rgb(0xbe, 0x18, 0x5d),
            User::Amon8808 => egui::Color32::from_rgb(0x03, 0x69, 0xa1),
            User::MathIsMath => egui::Color32::from_rgb(0x0f, 0x76, 0x6e),
            User::Urgalurga => egui::Color32::from_rgb(0xc2, 0x41, 0x0c),
            User::TheColdPanda => egui::Color32::from_rgb(0xb9, 0x1c, 0x1c),
        }
    }
}

const REQUEST_BODY: &str = r#"
{
    "fields": [
        "quantity",
        "card__oracleCard__name",
        "card__edition__editioncode",
        "card__multiverseid",
        "card__uid",
        "card__prices__cm"
    ],
    "page": 1,
    "game": 1,
    "pageSize": 10000
}
"#;

pub fn get_collections() -> anyhow::Result<Collection> {
    let mut collections = Collection::with_capacity(1000 * User::VARIANTS.len());

    for user in User::VARIANTS {
        let client = Client::new();

        let data: Response = client
            .post(format!(
                "https://archidekt.com/api/collection/export/v2/{}/",
                user.id()
            ))
            .header("Content-Type", "application/json")
            .body(REQUEST_BODY)
            .send()?
            .json()?;

        let mut reader = csv::Reader::from_reader(Cursor::new(data.content));

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
