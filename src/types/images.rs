use serde::{Serialize, Deserialize};

pub struct ConstImage {
    pub path: Option<&'static str>,
    /// Any valid CSS units (em, px, in...)
    pub width: &'static str,
    pub height: &'static str,
}

impl ConstImage {
    pub fn to_image(&self) -> Image {
        Image {
            path: self.path.map(|p| p.to_string()),
            width: self.width.to_string(),
            height: self.height.to_string(),
        }
    }
}

impl Default for ConstImage {
    fn default() -> Self {
        Self {
            path: None,
            width: "400px",
            height: "400px",
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Image {
    pub path: Option<String>,
    /// Any valid CSS units (em, px, in...)
    pub width: String,
    pub height: String,
}
