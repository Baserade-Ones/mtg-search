#[derive(serde::Deserialize)]
struct BulkObject {
    #[serde(rename = "type")]
    ty: String,
    download_uri: String,
}

#[derive(serde::Deserialize)]
struct BulkList {
    data: Vec<BulkObject>,
}

#[derive(serde::Deserialize)]
struct ScryfallObject {
    id: String,
    #[serde(default)]
    type_line: String,
    #[serde(default)]
    color_identity: Vec<String>,
}

fn fetch_oracle_url() -> anyhow::Result<String> {
    let req = ehttp::Request::get("https://api.scryfall.com/bulk-data");

    let bulk: BulkList = ehttp::fetch_blocking(&req)
        .map_err(anyhow::Error::msg)?
        .json()?;

    bulk.data
        .into_iter()
        .find_map(|obj| (obj.ty == "default_cards").then_some(obj.download_uri))
        .ok_or_else(|| anyhow::Error::msg("failed to find oracle_cards"))
}

pub struct Card {
    pub ty: String,
    pub color_identity: String,
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
            obj.color_identity.sort_by_key(|c| match c.as_str() {
                "W" => 1,
                "U" => 2,
                "B" => 3,
                "R" => 4,
                "G" => 5,
                _ => 0,
            });
            (
                obj.id,
                Card {
                    ty,
                    color_identity: obj.color_identity.join(""),
                },
            )
        })
        .collect())
}
