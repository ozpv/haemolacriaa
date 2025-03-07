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
        for (name, value) in i {
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

/// alias for Result<T, ServerFnError>
pub type Result<T, E = server_fn::ServerFnError> = std::result::Result<T, E>;

// stuff is used but it says that it isn't
#[allow(unused_macros)]
macro_rules! err {
    ($s:tt) => {
        Err(server_fn::ServerFnError::new($s))
    };

    ($s:tt, $c:expr) => {
        leptos::expect_context::<ResponseOptions>().set_status($c);
        Err(server_fn::ServerFnError::new($s))
    };
}

#[allow(unused_imports)]
pub(crate) use err;
