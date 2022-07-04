extern crate reqwest;

pub use reqwest::Error;

#[cfg(feature = "async")]
pub use ::reqwest::*;

#[cfg(feature = "blocking")]
pub use reqwest::blocking::*;

#[cfg(all(feature = "async", feature = "blocking"))]
compile_error!(
    "`client-reqwest` and `client-ureq` features cannot both be enabled at \
    the same time, if you want to use `client-ureq` you need to set \
    `default-features = false`"
);

#[cfg(not(any(feature = "async", feature = "blocking")))]
compile_error!(
    "You have to enable at least one of the available clients with the \
    `client-reqwest` or `client-ureq` features."
);