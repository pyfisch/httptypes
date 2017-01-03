use std::io::{self, Write};
use std::str;
use std::time::{Duration, SystemTime};

use httpdate::{parse_http_date, fmt_http_date};

use header::{Header, RequestHeader, ResponseHeader, parse_value, serialize_value,
            parse_list1, serialize_list};
use header::item::{HeaderField, Url};

header!{
    /// `Max-Forwards header`, [RFC7231 Section 5.1.2]
    pub struct MaxForwards(u32);
    (RequestHeader);
    NAME = "Max-Forwards";
    SENSITIVE = false;
    parse(s, _base) {
        parse_value::<u32>(s).map(Into::into)
    }
    serialize(self, iter) {
        serialize_value(iter, &self.0)
    }
}

header!{
    /// `Location` header, [RFC7231 Section 7.1.2]
    pub struct Location(Url);
    (ResponseHeader);
    NAME = "Location";
    SENSITIVE = false;
    parse(s, base) {
        if s.len() != 1 {
            return Err(());
        }
        let raw = str::from_utf8(&s[0]).map_err(|_| ())?;
        base.join(raw).map_err(|_| ()).map(Into::into)
    }
    serialize(self, iter) {
        serialize_value(iter, &self.0)
    }
}

/// `Retry-After` header, [RFC7231 Section 7.1.3]
#[derive(Clone, Debug)]
pub enum RetryAfter {
    /// A timestamp.
    Date(SystemTime),
    /// A duration.
    Delay(Duration),
}

impl ResponseHeader for RetryAfter {}

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

header!{
    /// `Vary` header, [RFC7231 Section Section 7.1.4]
    pub struct Vary(Vec<HeaderField>);
    (ResponseHeader);
    NAME = "Vary";
    SENSITIVE = false;
    parse(s, _base) {
        parse_list1(s).map(Into::into)
    }
    serialize(self, iter) {
        serialize_list(iter, &self.0[..])
    }
}
