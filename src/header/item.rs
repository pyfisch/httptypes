use std::ascii::AsciiExt;
use std::fmt::{self, Display};
use std::str::FromStr;

pub use charsets::Charset;
pub use url::Url;

pub type MediaType = String;
pub type Language = String;

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

#[derive(Clone, Debug)]
pub struct Quality<T> {
    item: T,
    weight: Weight,
}

impl<T> Quality<T> {
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
#[derive(Clone, Debug)]
pub struct Weight(u16);

impl Weight {
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
                    return Some(weight);
                }
                Some(_) => return None,
                None => return Some(0),
            }
        }
        Some('1') => {
            match iter.next() {
                Some('.') => {
                    while let Some(char) = iter.next() {
                        if char != '0' {
                            return None;
                        }
                    }
                    return Some(1000);
                }
                Some(_) => return None,
                None => return Some(1000),
            }
        }
        _ => return None,
    }
}
