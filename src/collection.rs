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
    Wantlist(String, Option<User>),
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
        Search::Wantlist(String::new(), None)
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
            Search::Wantlist(list, owner) => list.to_lowercase().lines().any(|want| {
                (owner.is_none() || owner.is_some_and(|owner| owner == data.owner))
                    && data.name.to_lowercase().contains(want)
            }),
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

pub struct CardDeduper<'a, I>
where
    I: Iterator<Item = &'a Entry>,
{
    on: bool,
    iter: std::iter::Peekable<I>,
}

impl<'a, I> Iterator for CardDeduper<'a, I>
where
    I: Iterator<Item = &'a Entry>,
{
    type Item = Entry;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.on {
            return self.iter.next().cloned();
        }

        match self.iter.next().cloned() {
            Some(mut head) => {
                while let Some(next) = self
                    .iter
                    .next_if(|next| next.name == head.name && next.owner == head.owner)
                {
                    head.quantity += next.quantity;
                    if next.set != head.set {
                        head.set.clear();
                        head.price = -1.0;
                    }
                }
                Some(head)
            }
            None => None,
        }
    }
}

pub trait CardDeduperExt<'a>
where
    Self: Sized + Iterator<Item = &'a Entry>,
{
    fn dedup_cards(self, on: bool) -> CardDeduper<'a, Self> {
        CardDeduper {
            on,
            iter: self.peekable(),
        }
    }
}

impl<'a, I> CardDeduperExt<'a> for I where I: Sized + Iterator<Item = &'a Entry> {}
