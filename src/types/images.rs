use serde::{Deserialize, Serialize};
use std::borrow::ToOwned;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
/// Any valid CSS units (em, px, in...)
/// Defaults to what makes sense (Usually 400px)
pub struct Image<T = &'static str> {
    /// This is the path in the URI, not a path on the server
    pub path: T,
    pub width: Option<T>,
    pub height: Option<T>,
}

impl<'a> From<Image<&'a str>> for Image<String> {
    fn from(i: Image<&'a str>) -> Image<String> {
        Image {
            path: i.path.to_string(),
            width: i.width.map(ToOwned::to_owned),
            height: i.height.map(ToOwned::to_owned),
        }
    }
}
