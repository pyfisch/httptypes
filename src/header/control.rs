use std::io::{self, Write};
use std::ops::Deref;
use std::str;
use std::time::{Duration, SystemTime};

use httpdate::{parse_http_date, fmt_http_date};

use header::{Header, RequestHeader, ResponseHeader, parse_value, serialize_value,
            parse_list1, serialize_list};
use header::item::{HeaderField, Url};

/// `Max-Forwards header`, [RFC7231 Section 5.1.2]
#[derive(Clone, Debug)]
pub struct MaxForwards(u32);

/// `Location` header, [RFC7231 Section 7.1.2]
#[derive(Clone, Debug)]
pub struct Location(Url);

/// `Retry-After` header, [RFC7231 Section 7.1.3]
#[derive(Clone, Debug)]
pub enum RetryAfter {
    /// A timestamp.
    Date(SystemTime),
    /// A duration.
    Delay(Duration),
}

/// `Vary` header, [RFC7231 Section Section 7.1.4]
#[derive(Clone, Debug)]
pub struct Vary(Vec<HeaderField>);

impl RequestHeader for MaxForwards {}

impl ResponseHeader for Location {}
impl ResponseHeader for RetryAfter {}
impl ResponseHeader for Vary {}

impl Header for MaxForwards {
    const NAME: &'static str = "Max-Forwards";
    const SENSITIVE: bool = false;

    fn parse(s: &[Vec<u8>], _base: Url) -> Result<Self, ()> {
        parse_value::<u32>(s).map(Into::into)
    }

    fn serialize<I: Iterator<Item = W>, W: Write>(&self, iter: I) -> io::Result<()> {
        serialize_value(iter, &self.0)
    }
}

impl Header for Location {
    const NAME: &'static str = "Location";
    const SENSITIVE: bool = false;

    fn parse(s: &[Vec<u8>], base: Url) -> Result<Self, ()> {
        if s.len() != 1 {
            return Err(());
        }
        let raw = str::from_utf8(&s[0]).map_err(|_| ())?;
        base.join(raw).map_err(|_| ()).map(Into::into)
    }

    fn serialize<I: Iterator<Item = W>, W: Write>(&self, iter: I) -> io::Result<()> {
        serialize_value(iter, &self.0)
    }
}

impl Header for RetryAfter {
    const NAME: &'static str = "Retry-After";
    const SENSITIVE: bool = false;

    fn parse(s: &[Vec<u8>], _base: Url) -> Result<Self, ()> {
        if s.len() != 1 {
            return Err(());
        }
        let raw = str::from_utf8(&s[0]).map_err(|_| ())?;
        if let Ok(date) = parse_http_date(raw) {
            return Ok(date.into())
        }
        let secs = raw.parse().map_err(|_| ())?;
        Ok(Duration::from_secs(secs).into())
    }

    fn serialize<I: Iterator<Item = W>, W: Write>(&self, iter: I) -> io::Result<()> {
        use self::RetryAfter::*;
        match *self {
            Date(x) => serialize_value(iter, fmt_http_date(x)),
            Delay(x) => serialize_value(iter, x.as_secs()),
        }
    }
}

impl Header for Vary {
    const NAME: &'static str = "Vary";
    const SENSITIVE: bool = false;

    fn parse(s: &[Vec<u8>], _base: Url) -> Result<Self, ()> {
        parse_list1(s).map(Into::into)
    }

    fn serialize<I: Iterator<Item = W>, W: Write>(&self, iter: I) -> io::Result<()> {
        serialize_list(iter, &self.0[..])
    }
}

impl Deref for MaxForwards {
    type Target = u32;

    fn deref(&self) ->  &Self::Target {
        &self.0
    }
}

impl Deref for Location {
    type Target = Url;

    fn deref(&self) ->  &Self::Target {
        &self.0
    }
}

impl Deref for Vary {
    type Target = Vec<HeaderField>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u32> for MaxForwards {
    fn from(t: u32) -> Self {
        MaxForwards(t)
    }
}

impl From<Url> for Location {
    fn from(t: Url) -> Self {
        Location(t)
    }
}

impl From<SystemTime> for RetryAfter {
    fn from(t: SystemTime) -> Self {
        RetryAfter::Date(t)
    }
}

impl From<Duration> for RetryAfter {
    fn from(t: Duration) -> Self {
        RetryAfter::Delay(t)
    }
}

impl From<Vec<HeaderField>> for Vary {
    fn from(t: Vec<HeaderField>) -> Self {
        Vary(t)
    }
}
