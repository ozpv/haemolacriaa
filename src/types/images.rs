use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::Type;
use std::borrow::ToOwned;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Type))]
pub struct Image<T = String> {
    pub path: T,
    /// Any valid CSS units (em, px, in...)
    /// Defaults to what makes sense (Usually 400px)
    pub width: Option<T>,
    pub height: Option<T>,
}

impl<'a> From<Image<&'a str>> for Image<String> {
    fn from(i: Image<&'a str>) -> Image<String> {
        Image {
            path: i.path.to_owned(),
            width: i.width.map(ToOwned::to_owned),
            height: i.height.map(ToOwned::to_owned),
        }
    }
}
