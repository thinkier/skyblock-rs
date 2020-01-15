extern crate base64;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate futures;
extern crate hyper;
extern crate hyper_tls;

mod http;
#[cfg(test)]
mod tests;
mod objects;

pub use http::{SkyblockApi, ApiError};
