extern crate base64;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[cfg(feature = "client")]
extern crate futures;
#[cfg(feature = "client")]
extern crate hyper;
#[cfg(feature = "client")]
extern crate hyper_tls;

#[cfg(feature = "nbt")]
extern crate nbt;

#[cfg(feature = "client")]
pub mod http;
#[cfg(test)]
mod tests;
pub mod objects;

#[cfg(feature = "client")]
pub use http::{SkyblockApi, ApiError, Key};
pub use objects::auction::{Auction, PartialAuction};
pub use objects::profile::PartialProfile;
use std::error::Error;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Box<dyn Error>>;
