//! Definitions for error types used throught the crate

use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
#[error(
    "{message}{path}",
    path = path.as_ref().map(|p| format!("\npath: {}", p.display())).unwrap_or("".to_owned()),
)]
pub struct Error {
    // NOTE(alexmozaidze): There is not a single instance of the message being dynamic in this crate,
    // so if there occurs an instance where the message is to be determined at runtime, then
    // use `Cow<'static, str>`.
    //
    // One could also use generics, but that would make the error type much more complex to use
    // and to reason about for the user.
    pub(crate) message: &'static str,
    pub(crate) path: Option<PathBuf>,
}

impl Error {
    pub(crate) const fn new(message: &'static str, path: Option<PathBuf>) -> Self {
        Self { message, path }
    }
}

macro_rules! error {
    ($message:expr $(,)?) => {{
        $crate::core::error::Error::new($message, ::core::option::Option::None)
    }};
    ($message:expr, $path:expr $(,)?) => {{
        $crate::core::error::Error::new(
            $message,
            ::core::option::Option::Some(::std::path::PathBuf::from($path)),
        )
    }};
}

pub(crate) use error;
