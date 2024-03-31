use std::{fmt::Display, num::ParseFloatError, str::FromStr};

use diesel::data_types::Cents;
use serde::{Deserialize, Serialize};

const CENTS: f64 = 100.0;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Default)]
pub struct Moneys(pub i64);

impl Display for Moneys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use rusty_money::{iso, Money};

        write!(f, "{}", Money::from_minor(self.0, iso::RUB))
    }
}

impl FromStr for Moneys {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s
            .split(|c| char::is_ascii_punctuation(&c))
            .collect::<Vec<&str>>()
            .into_iter()
            .peekable();

        let mut whole = String::default();
        let mut decimal = String::default();

        while let Some(num) = it.next() {
            if it.peek().is_some() || whole.is_empty() {
                whole.extend(num.chars());
            } else {
                decimal.extend(num.chars());
            }
        }

        let f_val: f64 = format!("{whole}.{decimal}").parse()?;

        let inner = f_val * CENTS;

        Ok(Self(inner as i64))
    }
}

impl From<Cents> for Moneys {
    fn from(value: Cents) -> Self {
        Self(value.0)
    }
}

impl Into<f64> for Moneys {
    fn into(self) -> f64 {
        self.0 as f64 / CENTS
    }
}
