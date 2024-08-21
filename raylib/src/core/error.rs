//! Definitions for error types used throught the crate

use std::{borrow::Cow, path::PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
#[error(
    "{message}{path}",
    path = path.as_ref().map(|p| format!("\npath: {}", p.display())).unwrap_or("".to_owned()),
)]
pub struct Error {
    pub(crate) message: Cow<'static, str>,
    pub(crate) path: Option<PathBuf>,
}

impl Error {
    pub(crate) const fn new(message: Cow<'static, str>, path: Option<PathBuf>) -> Self {
        Self { message, path }
    }
}

macro_rules! error {
    ($message:literal) => {{
        $crate::core::error::Error::new(
            std::borrow::Cow::Borrowed($message),
            ::core::option::Option::None,
        )
    }};
    ($message:expr $(,)?) => {{
        $crate::core::error::Error::new($message, ::core::option::Option::None)
    }};
    ($message:literal, $path:expr $(,)?) => {{
        $crate::core::error::Error::new(
            std::borrow::Cow::Borrowed($message),
            ::core::option::Option::Some(::std::path::PathBuf::from($path)),
        )
    }};
    ($message:expr, $path:expr $(,)?) => {{
        $crate::core::error::Error::new(
            $message,
            ::core::option::Option::Some(::std::path::PathBuf::from($path)),
        )
    }};
}

pub(crate) use error;
