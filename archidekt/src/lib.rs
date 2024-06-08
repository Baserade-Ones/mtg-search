#![allow(unused)]

use strum::{Display, FromRepr, VariantArray};

#[derive(Debug, Clone, serde::Deserialize)]
struct Response {
    content: String,
    #[serde(rename = "totalRows")]
    rows: u16,
    #[serde(rename = "moreContent")]
    more: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VariantArray, Display, FromRepr)]
pub enum User {
    Strosel = 331139,
    Amon8808 = 324351,
    //TODO MathIsMath = 0,
    Urgalurga = 382090,
    TheColdPanda = 418756,
}

impl User {
    fn id(&self) -> u32 {
        *self as u32
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

#[derive(serde::Serialize)]
struct Body {
    fields: &'static [&'static str],
    page: u8,
    game: u8,
    #[serde(rename = "pageSize")]
    size: u32,
}

#[cfg(not(target_arch = "wasm32"))]
///Gets a user's collection as a CSV String
pub fn get_collections(owner: &User) -> anyhow::Result<String> {
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
                "card__multiverseid",
                "card__uid",
                "card__prices__cm",
            ],
            page: 1,
            game: 1,
            size: 10000,
        },
    )?;

    let data: Response = ehttp::fetch_blocking(&req)
        .map_err(anyhow::Error::msg)?
        .json()?;

    Ok(data.content)
}
