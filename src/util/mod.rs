/// Defaults for serde.
pub mod defaults;
/// Error responses.
pub mod error;
/// Url building.
pub mod url;
pub use error::RouxError;
/// Options
pub mod option;
pub use option::FeedOption;
pub use option::TimePeriod;
