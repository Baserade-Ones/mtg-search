#![allow(unused)]

use strum::{Display, FromRepr, VariantArray};

mod color;
pub use color::ColorIdent;

#[derive(Debug, Clone, serde::Deserialize)]
struct Response {
    content: String,
    #[serde(rename = "totalRows")]
    rows: u16,
    #[serde(rename = "moreContent")]
    more: bool,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    VariantArray,
    Display,
    FromRepr,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum User {
    Strosel = 331139,
    Amon8808 = 324351,
    MathIsMath = 358259,
    Urgalurga = 382090,
    TheColdPanda = 418756,
    VikinGG = 454044,
    //Elin = 000
    OliverDizz = 603907,
}

impl User {
    fn id(&self) -> u32 {
        *self as u32
    }
}

#[derive(serde::Serialize)]
struct Body {
    fields: &'static [&'static str],
    page: u8,
    game: u8,
    #[serde(rename = "pageSize")]
    size: u32,
}

#[derive(serde::Deserialize)]
struct RawEntry {
    #[serde(rename = "Quantity")]
    quantity: u8,
    #[serde(rename = "Identities")]
    color_identity: ColorIdent,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Super-types")]
    supertypes: String,
    #[serde(rename = "Types")]
    types: String,
    #[serde(rename = "Edition Code")]
    set: String,
    #[serde(rename = "Scryfall ID")]
    scryfall: String,
    #[serde(rename = "Price (Card Market)")]
    price: f32,
}

pub type Collection = Vec<Entry>;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Entry {
    pub owner: User,
    pub quantity: u8,
    pub color_identity: ColorIdent,
    pub name: String,
    pub ty: String,
    pub set: String,
    pub scryfall: String,
    pub price: f32,
}

impl Entry {
    pub fn headers() -> impl Iterator<Item = &'static str> {
        [
            "Owner", "X", "Color Id", "Name", "Type", "Set", "Scryfall", "Price",
        ]
        .into_iter()
    }

    pub fn values(&self) -> impl Iterator<Item = (&'static str, String)> {
        Self::headers().zip([
            self.owner.to_string(),
            self.quantity.to_string(),
            self.color_identity.into(),
            self.name.clone(),
            self.ty.clone(),
            self.set.clone(),
            self.scryfall.clone(),
            format!("{:.2}€", self.price),
        ])
    }
}

#[cfg(not(target_arch = "wasm32"))]
///Gets a user's collection as a CSV String
pub fn get_collections(owner: &User) -> anyhow::Result<Collection> {
    let req = ehttp::Request::json(
        format!(
            "https://archidekt.com/api/collection/export/v2/{}/",
            owner.id()
        ),
        &Body {
            fields: &[
                "quantity",
                "card__oracleCard__name",
                "card__edition__editioncode",
                "card__uid",
                "card__prices__cm",
                "card__supertypes",
                "card__types",
                "card__colorIdentity",
            ],
            page: 1,
            game: 1,
            size: 10000,
        },
    )?;

    let data: Response = ehttp::fetch_blocking(&req)
        .map_err(anyhow::Error::msg)?
        .json()?;

    let mut reader = csv::Reader::from_reader(std::io::Cursor::new(data.content));

    let col = reader
        .deserialize::<RawEntry>()
        .map(|ent| {
            let RawEntry {
                quantity,
                name,
                set,
                scryfall,
                price,
                color_identity,
                supertypes,
                types,
            } = ent.map_err(anyhow::Error::msg)?;
            let ty = if supertypes.is_empty() {
                types
            } else {
                format!("{supertypes},{types}")
            };

            Ok(Entry {
                owner: *owner,
                quantity,
                color_identity,
                name,
                ty,
                set,
                scryfall,
                price,
            })
        })
        .collect::<anyhow::Result<Vec<Entry>>>()?;

    Ok(col)
}
