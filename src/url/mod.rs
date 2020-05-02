//! Re-exports Url and Host from `url` crate with some additional features
//! needed for Uption.
mod host;
#[allow(clippy::module_inception)]
mod url;

pub use self::url::HttpUrl;
pub use ::url::Url;
pub use host::Host;
