use anyhow::anyhow;

use super::ColorIdent;

#[derive(serde::Deserialize)]
struct BulkObject {
    #[serde(rename = "type")]
    ty: String,
    download_uri: String,
}

#[derive(serde::Deserialize)]
struct ScryfallObject {
    id: String,
    name: String,
    #[serde(default)]
    type_line: String,
    color_identity: ColorIdent,
}

fn fetch_oracle_url() -> anyhow::Result<String> {
    let req = ehttp::Request::get("https://api.scryfall.com/bulk-data/oracle-cards");
    let bulk: BulkObject = ehttp::fetch_blocking(&req)
        .map_err(anyhow::Error::msg)?
        .json()?;
    Ok(bulk.download_uri)
}

#[derive(Clone)]
pub struct Card {
    pub ty: String,
    pub color_identity: ColorIdent,
}

pub fn fetch_oracle_cards() -> anyhow::Result<std::collections::HashMap<String, Card>> {
    let req = ehttp::Request::get(fetch_oracle_url()?);

    let bulk: Vec<ScryfallObject> = ehttp::fetch_blocking(&req)
        .map_err(anyhow::Error::msg)?
        .json()?;

    Ok(bulk
        .into_iter()
        .map(|mut obj| {
            let ty = obj
                .type_line
                .split_once('—')
                .map(|(ty, _)| ty)
                .unwrap_or(&obj.type_line)
                .into();
            (
                obj.name,
                Card {
                    ty,
                    color_identity: obj.color_identity,
                },
            )
        })
        .collect())
}
