use serde::{de::Error, Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SizeError {
    #[error("Failed to parse Size from &str")]
    ParseError,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Size {
    XS,
    S,
    M,
    L,
    XL,
    XXL,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            Self::XS => "XS",
            Self::S => "S",
            Self::M => "M",
            Self::L => "L",
            Self::XL => "XL",
            Self::XXL => "XXL",
        };
        write!(f, "{display}")
    }
}

impl TryFrom<&str> for Size {
    type Error = SizeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "XS" => Ok(Self::XS),
            "S" => Ok(Self::S),
            "M" => Ok(Self::M),
            "L" => Ok(Self::L),
            "XL" => Ok(Self::XL),
            "XXL" => Ok(Self::XXL),
            _ => Err(Self::Error::ParseError),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Product {
    name: String,
    price: i64,
    size: Size,
}

impl Serialize for Product {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}|{}|{}", self.name, self.price, self.size))
    }
}

impl<'de> Deserialize<'de> for Product {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let items = <&str>::deserialize(deserializer)?
            .split('|')
            .collect::<Vec<&str>>();

        if items.len() != 3 {
            return Err(D::Error::custom(
                "Product must have three values seperated by '|'",
            ));
        }

        let price = items[1].parse::<i64>().map_err(D::Error::custom)?;

        let size = Size::try_from(items[2]).map_err(D::Error::custom)?;

        Ok(Self {
            name: items[0].to_string(),
            price,
            size,
        })
    }
}

impl Product {
    /// `name`: the name of the product
    /// `price`: the price of the product in cents
    /// `size`: the size of the product
    pub fn new(name: &str, price: i64, size: Size) -> Self {
        Self {
            name: name.to_string(),
            price,
            size,
        }
    }

    pub fn get_price(&self) -> i64 {
        self.price
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_size(&self) -> &Size {
        &self.size
    }
}

