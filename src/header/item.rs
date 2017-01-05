//! Types used inside header fields.
//!
//! Headers are often constructed from complex types.
//! These types are defined in this module.

use std::ascii::AsciiExt;
use std::fmt::{self, Display};
use std::str::FromStr;

#[cfg(feature="negotiation")]
pub use charsets::Charset;
pub use language_tags::LanguageTag;
pub use media_types::MediaType;
pub use url::Url;

/// Content coding names, [RFC 7231, Section 3.1.2.1]
///
/// This shall not be used for `Transfer-Encoding`. Case is
/// ignored for all codings per spec.
///
/// Aliases `x-compress` and `x-gzip` are mapped to their canonical values.
///
/// * Source: [HTTP Content Coding Registry]
/// (http://www.iana.org/assignments/http-parameters/#content-coding)
/// * Revision: 2016-08-01
#[derive(Clone, Debug, Eq)]
pub enum Coding {
    /// br: Brotli Compressed Data Format, [RFC7932]
    Br,
    /// compress: LZW coding, [RFC7230, Section 4.2.1]
    Compress,
    /// deflate: "zlib" data format, [RFC7230, Section 4.2.2]
    Deflate,
    /// exi: W3C Efficient XML Interchange,
    /// [W3C Recommendation: Efficient XML Interchange (EXI) Format]
    /// (http://www.w3.org/TR/exi/)
    Exi,
    /// gzip: LZ77 coding with a 32-bit CRC, [RFC7230, Section 4.2.3]
    Gzip,
    /// identity: synonym for "no encoding" in `Accept-Encoding`.
    ///
    /// Must not occur in `Content-Encoding` header.
    Identity,
    /// pack200-gzip: Network Transfer Format for Java,
    /// [JSR 200: Network Transfer Format for Java]
    /// (http://www.jcp.org/en/jsr/detail?id=200)
    Pack200Gzip,
    /// Used for unregistered content codings.
    Unregistered(String),
}

impl Display for Coding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Coding::*;
        f.write_str(match *self {
            Br => "br",
            Compress => "compress",
            Deflate => "deflate",
            Exi => "exi",
            Gzip => "gzip",
            Identity => "identity",
            Pack200Gzip => "pack200-gzip",
            Unregistered(ref s) => s,
        })
    }
}

impl FromStr for Coding {
    type Err = ();
    fn from_str(s: &str) -> Result<Coding, ()> {
        use self::Coding::*;
        Ok(match s {
            s if s.eq_ignore_ascii_case("br") => Br,
            s if s.eq_ignore_ascii_case("compress") || s.eq_ignore_ascii_case("x-compress") => {
                Compress
            }
            s if s.eq_ignore_ascii_case("deflate") => Deflate,
            s if s.eq_ignore_ascii_case("exi") => Exi,
            s if s.eq_ignore_ascii_case("gzip") || s.eq_ignore_ascii_case("x-gzip") => Gzip,
            s if s.eq_ignore_ascii_case("identity") => Identity,
            s if s.eq_ignore_ascii_case("pack200-gzip") => Pack200Gzip,
            s => Unregistered(s.to_owned()),
        })
    }
}

impl PartialEq for Coding {
    fn eq(&self, other: &Coding) -> bool {
        use self::Coding::*;
        match (self, other) {
            (&Br, &Br) |
            (&Compress, &Compress) |
            (&Deflate, &Deflate) |
            (&Exi, &Exi) |
            (&Gzip, &Gzip) |
            (&Identity, &Identity) |
            (&Pack200Gzip, &Pack200Gzip) => true,
            (&Unregistered(ref a), &Unregistered(ref b)) => a.eq_ignore_ascii_case(b),
            _ => false,
        }
    }
}

/// Quality items are used on content negotiation headers.
///
/// They indicate relative preferences of the client.
///
/// To create a new quality item with a weight of 0.5 one can
/// use `Quality::new("item", 500)`. A most preferred item with
/// the weight of 1.0 can be created with `"item".into()` if
/// needed.
#[derive(Clone, Debug)]
pub struct Quality<T> {
    item: T,
    weight: Weight,
}

impl<T> Quality<T> {
    /// Constructs a new quality item with a given weight.
    pub fn new<I: Into<Weight>>(item: T, weight: I) -> Quality<T> {
        Quality {
            item: item,
            weight: weight.into(),
        }
    }
}

impl<T: Display> Display for Quality<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.item.fmt(f)?;
        let weight = self.weight.0;
        if weight == 0 {
            f.write_str("; q=0")
        } else if weight < 1000 {
            write!(f,
                   "; q=0.{}",
                   format!("{:03}", weight).trim_right_matches('0'))
        } else {
            Ok(())
        }
    }
}

impl<T: FromStr> FromStr for Quality<T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Quality<T>, ()> {
        let mut iter = s.rsplitn(2, ';').map(|x| x.trim());
        let mut raw_weight = iter.next();
        let raw_item = iter.next().unwrap_or_else(|| raw_weight.take().unwrap());
        if let Some(raw_weight) = raw_weight {
            if raw_weight.starts_with("q=") || raw_weight.starts_with("Q=") {
                let raw_weight = &raw_weight[2..];
                if raw_weight.len() <= 5 {
                    if let Some(weight) = parse_weight(s) {
                        return Ok(Quality {
                            item: raw_item.parse::<T>().map_err(|_| ())?,
                            weight: Weight::new(weight),
                        });
                    } else {
                        return Err(());
                    }
                }
            }
        }
        Ok(Quality::new(raw_item.parse().map_err(|_| ())?, 1000))
    }
}

impl<T> From<T> for Quality<T> {
    fn from(t: T) -> Quality<T> {
        Quality {
            item: t,
            weight: Weight::new(1000),
        }
    }
}

/// A weight used for values with a quality.
///
/// The value is between 0.000 and 1.000 represented
/// by the numbers 0 to 1000.
///
/// The weight is stored as an integer for more compact
/// storage and easier parsing and comparison.
///
/// A weight from the RFC is multiplied by 100 to get
/// its integer value.
#[derive(Clone, Debug)]
pub struct Weight(u16);

impl Weight {
    /// Constructs a new weight from an integer between 0 and 1000.
    pub fn new(n: u16) -> Weight {
        assert!(n <= 1000, "Weight must be 1000 or less.");
        Weight(n)
    }
}

impl From<u16> for Weight {
    fn from(n: u16) -> Weight {
        Weight::new(n)
    }
}

fn parse_weight(s: &str) -> Option<u16> {
    let mut iter = s.chars();
    match iter.next() {
        Some('0') => {
            match iter.next() {
                Some('.') => {
                    let mut weight = 0;
                    while let Some(char) = iter.next() {
                        if char <= '0' || char >= '9' {
                            return None;
                        }
                        weight *= 10;
                        weight += char as u16 - '0' as u16;
                    }
                    Some(weight)
                }
                Some(_) => None,
                None => Some(0),
            }
        }
        Some('1') => {
            match iter.next() {
                Some('.') => {
                    for c in iter {
                        if c != '0' {
                            return None;
                        }
                    }
                    Some(1000)
                }
                Some(_) => None,
                None => Some(1000),
            }
        }
        _ => None,
    }
}

/// A header field name.
///
/// Header field names are case-insensitive.
#[derive(Clone, Debug)]
pub struct HeaderField(String);

impl FromStr for HeaderField {
    type Err = ();

    fn from_str(s: &str) -> Result<HeaderField, ()> {
        Ok(HeaderField(s.to_owned()))
    }
}

impl Display for HeaderField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

// check that each char in the slice is either:
// 1. %x21, or
// 2. in the range %x23 to %x7E, or
// 3. above %x80
fn check_slice_validity(slice: &str) -> bool {
    slice.bytes().all(|c|
        c == b'\x21' || (c >= b'\x23' && c <= b'\x7e') | (c >= b'\x80'))
}

/// An entity tag, defined in [RFC7232](https://tools.ietf.org/html/rfc7232#section-2.3)
///
/// An entity tag consists of a string enclosed by two literal double quotes.
/// Preceding the first double quote is an optional weakness indicator,
/// which always looks like `W/`. Examples for valid tags are `"xyzzy"` and `W/"xyzzy"`.
///
/// # ABNF
/// ```plain
/// entity-tag = [ weak ] opaque-tag
/// weak       = %x57.2F ; "W/", case-sensitive
/// opaque-tag = DQUOTE *etagc DQUOTE
/// etagc      = %x21 / %x23-7E / obs-text
///            ; VCHAR except double quotes, plus obs-text
/// ```
///
/// # Comparison
/// To check if two entity tags are equivalent in an application always use the `strong_eq` or
/// `weak_eq` methods based on the context of the Tag. Only use `==` to check if two tags are
/// identical.
///
/// The example below shows the results for a set of entity-tag pairs and
/// both the weak and strong comparison function results:
///
/// | ETag 1  | ETag 2  | Strong Comparison | Weak Comparison |
/// |---------|---------|-------------------|-----------------|
/// | `W/"1"` | `W/"1"` | no match          | match           |
/// | `W/"1"` | `W/"2"` | no match          | no match        |
/// | `W/"1"` | `"1"`   | no match          | match           |
/// | `"1"`   | `"1"`   | match             | match           |
// License note: All EntityTag code, documentation and impls were
// taken from https://github.com/hyperium/hyper/tree/bf2b8f licensed under the MIT license.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntityTag {
    /// Weakness indicator for the tag
    pub weak: bool,
    /// The opaque string in between the DQUOTEs
    tag: String
}

impl EntityTag {
    /// Constructs a new EntityTag.
    /// # Panics
    /// If the tag contains invalid characters.
    pub fn new(weak: bool, tag: String) -> EntityTag {
        assert!(check_slice_validity(&tag), "Invalid tag: {:?}", tag);
        EntityTag { weak: weak, tag: tag }
    }

    /// Constructs a new weak EntityTag.
    /// # Panics
    /// If the tag contains invalid characters.
    pub fn weak(tag: String) -> EntityTag {
        EntityTag::new(true, tag)
    }

    /// Constructs a new strong EntityTag.
    /// # Panics
    /// If the tag contains invalid characters.
    pub fn strong(tag: String) -> EntityTag {
        EntityTag::new(false, tag)
    }

    /// Get the tag.
    pub fn tag(&self) -> &str {
        self.tag.as_ref()
    }

    /// Set the tag.
    /// # Panics
    /// If the tag contains invalid characters.
    pub fn set_tag(&mut self, tag: String) {
        assert!(check_slice_validity(&tag), "Invalid tag: {:?}", tag);
        self.tag = tag
    }

    /// For strong comparison two entity-tags are equivalent if both are not weak and their
    /// opaque-tags match character-by-character.
    pub fn strong_eq(&self, other: &EntityTag) -> bool {
        !self.weak && !other.weak && self.tag == other.tag
    }

    /// For weak comparison two entity-tags are equivalent if their
    /// opaque-tags match character-by-character, regardless of either or
    /// both being tagged as "weak".
    pub fn weak_eq(&self, other: &EntityTag) -> bool {
        self.tag == other.tag
    }

    /// The inverse of `EntityTag.strong_eq()`.
    pub fn strong_ne(&self, other: &EntityTag) -> bool {
        !self.strong_eq(other)
    }

    /// The inverse of `EntityTag.weak_eq()`.
    pub fn weak_ne(&self, other: &EntityTag) -> bool {
        !self.weak_eq(other)
    }
}

impl Display for EntityTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.weak {
            write!(f, "W/\"{}\"", self.tag)
        } else {
            write!(f, "\"{}\"", self.tag)
        }
    }
}

impl FromStr for EntityTag {
    type Err = ();
    fn from_str(s: &str) -> Result<EntityTag, ()> {
        let length: usize = s.len();
        let slice = &s[..];
        // Early exits if it doesn't terminate in a DQUOTE.
        if !slice.ends_with('"') {
            return Err(());
        }
        // The etag is weak if its first char is not a DQUOTE.
        if slice.starts_with('"') && check_slice_validity(&slice[1..length-1]) {
            // No need to check if the last char is a DQUOTE,
            // we already did that above.
            return Ok(EntityTag { weak: false, tag: slice[1..length-1].to_owned() });
        } else if slice.starts_with("W/\"") && check_slice_validity(&slice[3..length-1]) {
            return Ok(EntityTag { weak: true, tag: slice[3..length-1].to_owned() });
        }
        Err(())
    }
}
