use ratatui::{
    text::Text,
    widgets::{Cell, Row},
};
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::io::Cursor;
use strum::VariantArray;

//TODO load collection from api

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
    pub fn as_row(&self, owner: User) -> Row {
        Row::new([
            owner.to_cell(),
            Cell::from(Text::from(format!("{}", self.quantity))),
            Cell::from(Text::from(self.name.clone())),
            Cell::from(Text::from(self.set.clone())),
            Cell::from(Text::from(self.multiverse.clone())),
            Cell::from(Text::from(self.scryfall.clone())),
            Cell::from(Text::from(format!("{:.2}€", self.price))),
        ])
    }

    pub fn matches(&self, search: String) -> bool {
        //TODO search syntax
        self.name.to_lowercase().contains(&search.to_lowercase())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::VariantArray, strum::Display)]
pub enum User {
    Strosel,
    Amon8808,
}

impl User {
    fn id(&self) -> u32 {
        match self {
            Self::Strosel => 331139,
            Self::Amon8808 => 324351,
        }
    }

    fn to_cell(self) -> Cell<'static> {
        use ratatui::style::palette::tailwind;
        Cell::from(
            Text::from(self.to_string()).style(ratatui::style::Style::default().fg(match self {
                Self::Strosel => tailwind::PINK.c700,
                Self::Amon8808 => tailwind::SKY.c700,
            })),
        )
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

pub fn get_collections() -> anyhow::Result<HashMap<User, Collection>> {
    let mut map = HashMap::<User, Vec<Entry>>::new();

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

        let col = reader
            .deserialize::<Entry>()
            .collect::<Result<Vec<Entry>, csv::Error>>()?;

        map.insert(*user, col);
    }

    Ok(map)
}
