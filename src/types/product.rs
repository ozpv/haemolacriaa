use serde::{de::Error, Deserialize, Serialize};

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Product {
    name: String,
    price: i64,
}

impl Serialize for Product {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}-{}", self.name, self.price))
    }
}

impl<'de> Deserialize<'de> for Product {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (name, price) = <&str>::deserialize(deserializer)?
            .split_once('-')
            .ok_or_else(|| D::Error::custom("Failed to split key"))?;

        let price = price.parse::<i64>().map_err(D::Error::custom)?;

        Ok(Self {
            name: name.to_string(),
            price,
        })
    }
}

impl Product {
    /// `name`: the name of the product
    /// `price`: the price of the product in cents
    pub fn new(name: &str, price: i64) -> Self {
        Self {
            name: name.to_string(),
            price,
        }
    }

    pub fn get_price(&self) -> i64 {
        self.price
    }

    pub fn get_name(self) -> String {
        self.name
    }
}
