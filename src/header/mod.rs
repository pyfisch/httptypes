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

use std::fmt::{Debug, Display};
use std::io::{self, Write};
use std::iter::Iterator;
use std::str::{self, FromStr};

use url::Url;

#[cfg(feature="context")]
pub use self::context::{From, Referer, UserAgent, Allow, Server};
#[cfg(feature="control")]
pub use self::control::{MaxForwards, Location, RetryAfter, Vary};
#[cfg(feature="metadata")]
pub use self::metadata::{ContentType, ContentEncoding, ContentLanguage, ContentLocation};
#[cfg(feature="negotiation")]
pub use self::negotiation::{Accept, AcceptCharset, AcceptEncoding, AcceptLanguage};

#[cfg(feature="context")]
mod context;
#[cfg(feature="control")]
mod control;
pub mod item;
#[cfg(feature="metadata")]
mod metadata;
#[cfg(feature="negotiation")]
mod negotiation;

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

fn parse_value<T: FromStr>(s: &[Vec<u8>]) -> Result<T, ()> {
    if s.len() != 1 {
        return Err(());
    }
    str::from_utf8(s[0].as_slice())
        .ok()
        .and_then(|x| x.parse().ok())
        .ok_or(())
}

fn serialize_value<I, W, T>(mut iter: I, v: T) -> Result<(), io::Error>
    where I: Iterator<Item = W>,
          W: Write,
          T: Display
{
    write!(iter.next().unwrap(), "{}", v)
}

struct IterListHeader<'a> {
    values: &'a [Vec<u8>],
    line: usize,
    column: usize,
}

impl<'a> IterListHeader<'a> {
    fn new(values: &[Vec<u8>]) -> IterListHeader {
        IterListHeader {
            values: values,
            line: 0,
            column: 0,
        }
    }
}

impl<'a> Iterator for IterListHeader<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<&'a [u8]> {
        for line in self.line..self.values.len() {
            let value = &self.values[line];
            let mut maybe_start_column = None;
            let mut end_column = 0;
            for column in self.column..value.len() {
                let byte = value[column];
                if byte != b' ' && byte != b'\t' && byte != b',' {
                    end_column = column + 1;
                    if maybe_start_column.is_none() {
                        maybe_start_column = Some(column)
                    }
                } else if byte == b',' {
                    if let Some(start_column) = maybe_start_column {
                        self.column = column + 1;
                        return Some(&value[start_column..end_column]);
                    }
                    maybe_start_column = None;
                }
            }
            self.line = line + 1;
            self.column = 0;
            if let Some(start_column) = maybe_start_column {
                return Some(&value[start_column..end_column]);
            }
        }
        None
    }
}

fn parse_list0<T: FromStr>(s: &[Vec<u8>]) -> Result<Vec<T>, ()> {
    let iter = IterListHeader::new(s);
    let items: Option<Vec<T>> = iter.map(|x| {
            str::from_utf8(x)
                .ok()
                .and_then(|x| x.parse().ok())
        })
        .collect();
    items.ok_or(())
}

fn parse_list1<T: FromStr>(s: &[Vec<u8>]) -> Result<Vec<T>, ()> {
    let list = try!(parse_list0(s));
    if list.is_empty() {
        return Err(());
    }
    Ok(list)
}

fn serialize_list<I, W, T>(mut iter: I, values: &[T]) -> Result<(), io::Error>
    where I: Iterator<Item = W>,
          W: Write,
          T: Display
{
    let mut w = iter.next().unwrap();
    for (i, v) in values.iter().enumerate() {
        if i != 0 {
            w.write_all(b", ")?;
        }
        write!(w, "{}", v)?;
    }
    Ok(())
}
