use base64::{engine::general_purpose, Engine};
use http::{HeaderMap, HeaderName, HeaderValue};

pub trait InsertMany {
    fn insert_many(&mut self, i: impl IntoIterator<Item = (HeaderName, HeaderValue)>);
}

impl InsertMany for HeaderMap<HeaderValue> {
    /// # Note
    ///
    /// Discards already existing values instead of returning them
    fn insert_many(&mut self, i: impl IntoIterator<Item = (HeaderName, HeaderValue)>) {
        for (name, value) in i.into_iter() {
            self.insert(name, value);
        }
    }
}

#[inline]
pub fn gen_rand_string<const N: usize>() -> String {
    let mut bytes = [0u8; N];

    rand::fill(&mut bytes[..]);

    general_purpose::STANDARD.encode(bytes)
}
