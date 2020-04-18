mod host;
#[allow(clippy::module_inception)]
mod url;

pub use self::url::HttpUrl;
pub use ::url::Url;
pub use host::Host;
