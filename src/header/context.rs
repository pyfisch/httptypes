use std::convert;
use std::io::{self, Write};
use std::ops::Deref;
use std::str;

use header::{Header, RequestHeader, ResponseHeader, parse_value, serialize_value, parse_list0,
             serialize_list};
use header::item::Url;
use Method;

/// `From` header, [RFC7231 Section 5.5.1]
#[derive(Clone, Debug)]
pub struct From(String);

/// `Referer` header, [RFC7231 Section 5.5.2]
#[derive(Clone, Debug)]
pub struct Referer(Url);

/// `User-Agent` header, [RFC7231 Section 5.5.3]
///
/// The value is decoded as UTF-8. Invalid bytes are replaced
/// with U+FFFD REPLACEMENT CHARACTER.
#[derive(Clone, Debug)]
pub struct UserAgent(String);

/// `Allow` header, [RFC7231 Section 7.4.1]
#[derive(Clone, Debug)]
pub struct Allow(Vec<Method>);

/// `Server` header, [RFC7231 Section 7.4.2]
///
/// The value is decoded as UTF-8. Invalid bytes are replaced
/// with U+FFFD REPLACEMENT CHARACTER.
#[derive(Clone, Debug)]
pub struct Server(String);

impl RequestHeader for From {}
impl RequestHeader for Referer {}
impl RequestHeader for UserAgent {}

impl ResponseHeader for Allow {}
impl ResponseHeader for Server {}

impl Header for From {
    const NAME: &'static str = "From";
    const SENSITIVE: bool = false;

    fn parse(s: &[Vec<u8>], _base: Url) -> Result<Self, ()> {
        parse_value::<String>(s).map(Into::into)
    }

    fn serialize<I: Iterator<Item = W>, W: Write>(&self, iter: I) -> io::Result<()> {
        serialize_value(iter, &self.0)
    }
}

impl Header for Referer {
    const NAME: &'static str = "Referer";
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

impl Header for UserAgent {
    const NAME: &'static str = "User-Agent";
    const SENSITIVE: bool = false;

    fn parse(s: &[Vec<u8>], _base: Url) -> Result<Self, ()> {
        if s.len() != 1 {
            return Err(());
        }
        Ok(String::from_utf8_lossy(&s[0]).into_owned().into())
    }

    fn serialize<I: Iterator<Item = W>, W: Write>(&self, iter: I) -> io::Result<()> {
        serialize_value(iter, &self.0)
    }
}

impl Header for Allow {
    const NAME: &'static str = "Allow";
    const SENSITIVE: bool = false;

    fn parse(s: &[Vec<u8>], _base: Url) -> Result<Self, ()> {
        parse_list0(s).map(Into::into)
    }

    fn serialize<I: Iterator<Item = W>, W: Write>(&self, iter: I) -> io::Result<()> {
        serialize_list(iter, &self.0)
    }
}

impl Header for Server {
    const NAME: &'static str = "Server";
    const SENSITIVE: bool = false;

    fn parse(s: &[Vec<u8>], _base: Url) -> Result<Self, ()> {
        if s.len() != 1 {
            return Err(());
        }
        Ok(String::from_utf8_lossy(&s[0]).into_owned().into())
    }

    fn serialize<I: Iterator<Item = W>, W: Write>(&self, iter: I) -> io::Result<()> {
        serialize_value(iter, &self.0)
    }
}

impl Deref for From {
    type Target = String;

    fn deref(&self) -> &String {
        &self.0
    }
}

impl Deref for Referer {
    type Target = Url;

    fn deref(&self) -> &Url {
        &self.0
    }
}

impl Deref for UserAgent {
    type Target = String;

    fn deref(&self) -> &String {
        &self.0
    }
}

impl Deref for Allow {
    type Target = Vec<Method>;

    fn deref(&self) -> &Vec<Method> {
        &self.0
    }
}

impl Deref for Server {
    type Target = String;

    fn deref(&self) -> &String {
        &self.0
    }
}

impl convert::From<String> for From {
    fn from(t: String) -> Self {
        From(t)
    }
}

impl convert::From<Url> for Referer {
    fn from(t: Url) -> Self {
        Referer(t)
    }
}

impl convert::From<String> for UserAgent {
    fn from(t: String) -> Self {
        UserAgent(t)
    }
}

impl convert::From<Vec<Method>> for Allow {
    fn from(t: Vec<Method>) -> Self {
        Allow(t)
    }
}

impl convert::From<String> for Server {
    fn from(t: String) -> Self {
        Server(t)
    }
}
