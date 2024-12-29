use serde::{Deserialize, Serialize};
use std::borrow::ToOwned;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
/// Dimensions default to what makes sense (Usually 400px)
pub struct Image<T = &'static str> {
    /// This is the path in the URI, not a path on the server
    pub path: T,
    /// width in px
    pub width: Option<T>,
    /// height in px
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
