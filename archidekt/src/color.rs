#[derive(Default, Debug, Clone, Copy, serde::Serialize)]
#[serde(into = "String")]
pub struct ColorIdent([bool; 5]);

impl ColorIdent {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn contains(&self, other: &Self) -> bool {
        /*
         * If color is always an ordered set of bools then given a target T
         * the candidate C must be true ONLY in positions where T is true
         *
         * T C
         * t t => t
         * t f => t
         * f t => f
         * f f => t
         */
        self.0.iter().zip(other.0.iter()).all(|(&t, &c)| t || !c)
    }
}

impl std::ops::Deref for ColorIdent {
    type Target = [bool; 5];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for ColorIdent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

use serde::{Deserialize, Deserializer};
impl<'de> Deserialize<'de> for ColorIdent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{self, SeqAccess, Visitor};
        struct StringOrSequence;

        impl<'de> Visitor<'de> for StringOrSequence {
            type Value = ColorIdent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string or sequence")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let mut color = ColorIdent::new();
                for c in value.chars() {
                    color[match c {
                        'W' | 'w' => 0,
                        'U' | 'u' => 1,
                        'B' | 'b' => 2,
                        'R' | 'r' => 3,
                        'G' | 'g' => 4,
                        _ => {
                            return Err(E::invalid_value(
                                de::Unexpected::Char(c),
                                &"One of [W, U, B, R, G]",
                            ))
                        }
                    }] = true;
                }
                Ok(color)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                self.visit_str::<A::Error>(
                    &std::iter::from_fn(|| seq.next_element::<char>().transpose())
                        .collect::<Result<String, _>>()?,
                )
            }
        }

        deserializer.deserialize_any(StringOrSequence)
    }
}

impl From<ColorIdent> for String {
    fn from(value: ColorIdent) -> Self {
        value
            .iter()
            .zip("WUBRG".chars())
            .filter_map(|(ok, c)| ok.then_some(c))
            .collect()
    }
}
