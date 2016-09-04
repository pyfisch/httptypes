//! The *httptypes* crate is a collection of useful abstractions for
//! building HTTP clients and servers.
//!
//! It contains types for
//!
//! * [request method](enum.Method.html),
//! * [response status](struct.Status.html),
//! * [header fields](header/index.html) and
//! * the [protocol version](enum.Version.html).
//!
//! Each type has useful that help to implement HTTP.

#![feature(associated_consts, question_mark)]
#![deny(missing_docs)]

#[cfg(feature="negotiation")]
extern crate charsets;
extern crate language_tags;
#[macro_use]
extern crate matches;
extern crate media_types;
extern crate url;

pub mod header;
mod method;
mod status;
mod util;
mod version;

pub use header::Header;
pub use method::Method;
pub use status::{Status, StatusClass};
pub use version::Version;
