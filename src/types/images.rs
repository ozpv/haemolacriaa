use serde::{Deserialize, Serialize};
use std::borrow::ToOwned;

/// Image type
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
pub struct Image<T = &'static str> {
    /// The image name and the extension
    pub name: T,
    /// width in px
    pub width: Option<T>,
    /// height in px
    pub height: Option<T>,
}

impl Image {
    pub fn cdn_path(&self) -> String {
        format!(
            "/assets/{}?width={}&height={}",
            &self.name,
            self.width.unwrap_or("400"),
            self.height.unwrap_or("400")
        )
    }
}

impl<'a> From<Image<&'a str>> for Image<String> {
    fn from(i: Image<&'a str>) -> Image<String> {
        Image {
            name: i.name.to_string(),
            width: i.width.map(ToOwned::to_owned),
            height: i.height.map(ToOwned::to_owned),
        }
    }
}
