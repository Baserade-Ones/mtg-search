use anyhow::anyhow;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use winnow::ascii::Caseless;
use winnow::combinator::{alt, opt, permutation, repeat, separated, terminated};
use winnow::error::{AddContext, ContextError, ParseError, StrContext};
use winnow::stream::Stream;
use winnow::Parser;

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

impl From<ColorIdent> for String {
    fn from(value: ColorIdent) -> Self {
        value
            .iter()
            .zip("WUBRG".chars())
            .filter_map(|(ok, c)| ok.then_some(c))
            .collect()
    }
}

impl<'de> Deserialize<'de> for ColorIdent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ColorIdentParser)
    }
}

struct SingleColorParser {
    long: String,
    short: String,
}

impl SingleColorParser {
    fn new(name: &str) -> Self {
        let long = name.to_lowercase();
        let short = name
            .chars()
            .find(|c| c.is_uppercase())
            .unwrap()
            .to_ascii_lowercase()
            .to_string();

        Self { long, short }
    }
}

impl<'s> winnow::Parser<&'s str, &'s str, ContextError> for SingleColorParser {
    fn parse_next(&mut self, input: &mut &'s str) -> winnow::Result<&'s str, ContextError> {
        alt([Caseless(self.long.as_str()), Caseless(self.short.as_str())]).parse_next(input)
    }
}

struct ColorIdentParser;

impl<'s> winnow::Parser<&'s str, ColorIdent, ContextError> for ColorIdentParser {
    fn parse_next(&mut self, input: &mut &'s str) -> winnow::Result<ColorIdent, ContextError> {
        //HACK use `repeat(_, terminated(_, opt(_))...)` becase `separated` can't handle empty separator
        let colors: Vec<usize> = repeat(
            ..=5,
            alt((
                terminated(SingleColorParser::new("White"), opt(',')).value(0),
                terminated(SingleColorParser::new("blUe"), opt(',')).value(1),
                terminated(SingleColorParser::new("Black"), opt(',')).value(2),
                terminated(SingleColorParser::new("Red"), opt(',')).value(3),
                terminated(SingleColorParser::new("Green"), opt(',')).value(4),
            )),
        )
        .parse_next(input)
        .map_err(|mut e: ContextError| {
            e.add_context(input, &input.checkpoint(), StrContext::Label("Here2"))
        })?;

        let mut ident = ColorIdent::new();
        for c in colors.into_iter() {
            ident[c] = true;
        }
        Ok(ident)
    }
}

impl<'de> Visitor<'de> for ColorIdentParser {
    type Value = ColorIdent;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("string or sequence")
    }

    fn visit_str<E>(mut self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.parse(value).map_err(|e| E::custom(e.to_string()))
    }
}
