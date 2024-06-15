pub struct Image {
    pub path: Option<&'static str>,
    /// Any valid CSS units (em, px, in...)
    pub width: &'static str,
    pub height: &'static str,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            path: None,
            width: "400px",
            height: "400px",
        }
    }
}
