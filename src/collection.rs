use archidekt::{Collection, Entry, User};
use std::collections::HashSet;
use strum::VariantArray;

pub enum Search {
    Single {
        owner: Option<User>,
        color: [bool; 5],
        name: String,
        ty: String,
    },
    Wantlist(String),
}

impl Search {
    pub fn single() -> Self {
        Self::Single {
            owner: None,
            color: [false; 5],
            name: String::new(),
            ty: String::new(),
        }
    }

    pub fn wantlist() -> Self {
        Search::Wantlist(String::new())
    }

    pub fn apply(&self, data: &Entry) -> bool {
        match self {
            Search::Single {
                owner,
                color,
                name,
                ty,
            } => [
                owner.map_or(true, |owner| owner == data.owner),
                color.iter().all(|x| !x)
                    || (&data.color_identity.chars().collect::<HashSet<_>>()
                        - &color
                            .iter()
                            .zip("WUBRG".chars())
                            .filter_map(|(on, c)| on.then_some(c))
                            .collect::<HashSet<_>>())
                        .is_empty(),
                data.name.to_lowercase().contains(&name.to_lowercase()),
                data.ty.to_lowercase().contains(&ty.to_lowercase()),
            ]
            .into_iter()
            .all(|x| x),
            Search::Wantlist(list) => list
                .to_lowercase()
                .lines()
                .any(|want| data.name.to_lowercase().contains(want)),
        }
    }
}

impl PartialEq for Search {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Single { .. }, Self::Single { .. })
                | (Self::Wantlist { .. }, Search::Wantlist { .. })
        )
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_collections() -> anyhow::Result<Collection> {
    let mut collections = Collection::with_capacity(1000 * User::VARIANTS.len());

    for user in User::VARIANTS {
        collections.append(&mut archidekt::get_collections(user)?);
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

        let mut reader = csv::Reader::from_reader(std::io::Cursor::new(data));

        let mut col = reader
            .deserialize::<Entry>()
            .collect::<Result<Vec<Entry>, csv::Error>>()?;

        collections.append(&mut col);
    }

    Ok(collections)
}
