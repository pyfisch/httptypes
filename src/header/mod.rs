//! HTTP header fields add information to messages.
//!
//! They are most likely the most complex construct of the protocol.
//! All header fields are key-value pairs: `Server: Apache/2.4.9 (Unix)`.
//! Some may contain multiple value separated by commas:
//! `Accept-Language: de-DE, de, en-US, en`.
//! Each header field has different syntax and semantics. These types
//! parse the header fields and serialize them.
//!
//! The header fields can be sorted into different groups.
//!
//! ## Conditional Requests
//! (needs to be written)
//!
//! ## Message Context
//!
//! Information about the resource and the endpoints.
//!
//! * [`From`](struct.From.html): client email address
//! * [`Referer`](struct.Referer.html): page visited before and
//!     linking to current resource
//! * [`User-Agent`](struct.UserAgent.html): client software used
//! * [`Allow`](struct.Allow.html): methods allowed on resource
//! * [`Server`](struct.Server.html): server software used
//!
//! ## Representation Metadata
//!
//! Properties of the transmitted representation. A *resource* is a
//! document in HTTP. It may have different *representations*. These
//! differ in their language, data type or compression.
//!
//! A client tell the server its preference for some representation
//! using [content negotiation](#content-negotiation) header fields.
//!
//! * [`Content-Type`](struct.ContentType.html): response media type
//! * [`Content-Encoding`](struct.ContentEncoding.html): compression applied
//!     to the representation
//! * [`Content-Language`](struct.ContentLanguage.html): user languages
//!     the resource is intended for
//! * [`Content-Location`](struct.ContentLocation.html): a link to the
//!     current representation of the resource
//!
//! ## Control Data
//! (needs to be written)
//!
//! ## Content Negotiation
//!
//! A client sends these header fields along with a request for some
//! resource so the server can give the client a response with the
//! properties preferred by the client.
//!
//! * [`Accept`](struct.Accept.html): preferred media types ("MIME types")
//! * [`Accept-Charset`](struct.AcceptCharset.html): preferred character encodings
//! * [`Accept-Encoding`](struct.AcceptEncoding.html): preferred content encodings
//!     usually used for compression
//! * [`Accept-Language`](struct.AcceptLanguage.html): preferred languages
//!     of the user
//!
//! ## Omitted header fields
//! While *httptypes* aims to support the common header fields some are
//! intentionally excluded. They usually can be better handled at a lower
//! protocol level.
//!
//! * `Date`: message creation date. Created automatically on the server.
//!     Unsure about common client usage.
//! * `Expect: 100-continue`: better handled at the syntax and
//!     routing layer.
//! * `MIME-Version`: unsure about usage and placement.

use std::fmt::Debug;
use std::io::{self, Write};
use std::iter::Iterator;
use std::str;

use url::Url;

#[cfg(feature="conditional")]
pub use self::conditional::{ETag, IfMatch, IfModifiedSince, IfNoneMatch, IfUnmodifiedSince, LastModified};
#[cfg(feature="context")]
pub use self::context::{From, Referer, UserAgent, Allow, Server};
#[cfg(feature="control")]
pub use self::control::{MaxForwards, Location, RetryAfter, Vary};
#[cfg(feature="metadata")]
pub use self::metadata::{ContentType, ContentEncoding, ContentLanguage, ContentLocation};
#[cfg(feature="negotiation")]
pub use self::negotiation::{Accept, AcceptCharset, AcceptEncoding, AcceptLanguage};
use self::util::*;

macro_rules! header {
    (
        $(#[$a:meta])*
        pub struct $header:ident($inner:ty);
        ($($usage:ty)*);
        NAME = $name:expr;
        SENSITIVE = $sensitive:expr;
        parse($s:ident, $base:ident) $parse:block
        serialize($self_:ident, $iter:ident) $serialize:block
    ) => {
        $(#[$a])*
        #[derive(Clone, Debug)]
        pub struct $header($inner);

        impl ::std::convert::From<$inner> for $header {
            fn from(t: $inner) -> $header {
                $header(t)
            }
        }

        impl ::std::convert::From<$header> for $inner {
            fn from(header: $header) -> $inner {
                header.0
            }
        }

        $(
            impl $usage for $header {}
        )*

        impl ::header::Header for $header {
            const NAME: &'static str = $name;
            const SENSITIVE: bool = $sensitive;

            fn parse($s: &[Vec<u8>], $base: ::url::Url) -> Result<Self, ()>
            $parse

            fn serialize<I: Iterator<Item = W>, W: ::std::io::Write>(&$self_, $iter: I)
                -> ::std::io::Result<()>
            $serialize
        }
    }
}

#[cfg(feature="conditional")]
mod conditional;
#[cfg(feature="context")]
mod context;
#[cfg(feature="control")]
mod control;
pub mod item;
#[cfg(feature="metadata")]
mod metadata;
#[cfg(feature="negotiation")]
mod negotiation;
pub mod util;

/// A HTTP header field.
///
/// This trait is used for deserialization and serialization
/// and to enforce some commen traits on header fields.
pub trait Header: Clone + Debug + Sized {
    /// The name of the header field.
    ///
    /// Although names are case-insensitive in HTTP/1.x
    /// and specified to be always lowercased in HTTP/2
    /// they should here be specified in their most
    /// common form: Camel-Case-With-Dashes eg.
    /// `Accept-Encoding`
    const NAME: &'static str;
    /// Mark a header field as possibly sensitive.
    ///
    /// HTTP/2 enables compression of header fields using
    /// a static and a dynamic table. Certain header fields
    /// like those used for authentication may be excluded
    /// from compression to avoid leaking information.
    const SENSITIVE: bool;

    /// Parses a request header fields from possibly multiple
    /// binary strings.
    ///
    /// An error indicates a parser failure. List headers
    /// should try to parse all values and silently ignore
    /// invalid ones.
    ///
    /// The base URL is the effective request URL and is used
    /// to parse relative URLs as commonly found in `Referer`
    /// and `Content-Location` headers to their absolute form.
    fn parse(s: &[Vec<u8>], base: Url) -> Result<Self, ()>;

    /// Serializes a header field value.
    ///
    /// The provided iterator emits a stream of writable objects.
    /// For each line written a new writeable object is requested
    /// from the iterator. Usually headers will only write a single
    /// line as most list headers use the comma to concatenate
    /// individual values. But the `Set-Cookie` header should
    /// use a separate line for each cookie. The iterator must
    /// always provide a new writeable object and never `None`.
    fn serialize<I: Iterator<Item = W>, W: Write>(&self, iter: I) -> Result<(), io::Error>;
}

/// Marker trait for request headers.
///
/// With this trait users of this library can statically
/// assure that a header is used correctly.
pub trait RequestHeader: Header {}

/// Marker trait for response headers.
///
/// With this trait users of this library can statically
/// assure that a header is used correctly.
pub trait ResponseHeader: Header {}
