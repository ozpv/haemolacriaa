use leptos::prelude::*;
use leptos_icons::Icon;
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

#[component]
pub fn SizeChartModal() -> impl IntoView {
    view! {
        <div tabindex="-1" class="fixed bg-crust-dark bg-opacity-80 max-h-full w-full h-full top-0 left-0 z-10 overflow-x-hidden overflow-y-auto p-4 md:inset-0" id="size-chart">
            <div class="relative w-full max-w-lg max-h-full">
                <div class="relative bg-base-dark rounded-lg w-full">
                    <div class="flex items-center justify-between border-b border-surface-dark rounded-t py-2">
                        <h3 class="text-text-dark font-sans text-xl py-3 px-8">
                            "size chart"
                        </h3>
                        <button class="text-overlay-dark-200 px-6">
                            <Icon icon={icondata::BsXLg} width="20px" height="20px" />
                        </button>
                    </div>
                    <div class="px-4 py-5">
                        <p class="text-text-dark font-inter">"Add the chart here"</p>
                    </div>
                    <div class="flex flex-row-reverse border-t border-surface-dark rounded-b px-4 py-3">
                        <button class="text-text-dark font-inter bg-surface-dark py-2 px-4 rounded hover:bg-surface-dark-100 hover:text-blue-dark">
                            "return"
                        </button>
                    </div>
                </div>
            </div>
        </div>

    }
}
