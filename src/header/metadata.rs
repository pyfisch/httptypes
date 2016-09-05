use std::io::{self, Write};
use std::ops::Deref;
use std::str;

use header::{Header, RequestHeader, ResponseHeader, parse_value, serialize_value, parse_list1,
             serialize_list};
use header::item::{MediaType, Coding, LanguageTag, Url};

/// `Content-Type` header, [RFC7231 Section 3.1.1.5]
#[derive(Clone, Debug)]
pub struct ContentType(MediaType);

/// `Content-Encoding` header, [RFC7231 Section 3.1.2.2]
#[derive(Clone, Debug)]
pub struct ContentEncoding(Vec<Coding>);

/// `Content-Language` header, [RFC7231 Section 3.1.3.2]
#[derive(Clone, Debug)]
pub struct ContentLanguage(Vec<LanguageTag>);

/// `Content-Location` header, [RFC7231 Section 3.1.4.2]
#[derive(Clone, Debug)]
pub struct ContentLocation(Url);

impl RequestHeader for ContentType {}
impl RequestHeader for ContentEncoding {}
impl RequestHeader for ContentLanguage {}
impl RequestHeader for ContentLocation {}

impl ResponseHeader for ContentType {}
impl ResponseHeader for ContentEncoding {}
impl ResponseHeader for ContentLanguage {}
impl ResponseHeader for ContentLocation {}

impl Header for ContentType {
    const NAME: &'static str = "Content-Type";
    const SENSITIVE: bool = false;

    fn parse(s: &[Vec<u8>], _base: Url) -> Result<Self, ()> {
        parse_value::<MediaType>(s).map(Into::into)
    }

    fn serialize<I: Iterator<Item = W>, W: Write>(&self, iter: I) -> io::Result<()> {
        serialize_value(iter, &self.0)
    }
}

impl Header for ContentEncoding {
    const NAME: &'static str = "Content-Encoding";
    const SENSITIVE: bool = false;

    fn parse(s: &[Vec<u8>], _base: Url) -> Result<Self, ()> {
        parse_list1(s).map(Into::into)
    }

    fn serialize<I: Iterator<Item = W>, W: Write>(&self, iter: I) -> io::Result<()> {
        serialize_list(iter, &self.0)
    }
}

impl Header for ContentLanguage {
    const NAME: &'static str = "Content-Language";
    const SENSITIVE: bool = false;

    fn parse(s: &[Vec<u8>], _base: Url) -> Result<Self, ()> {
        parse_list1(s).map(Into::into)
    }

    fn serialize<I: Iterator<Item = W>, W: Write>(&self, iter: I) -> io::Result<()> {
        serialize_list(iter, &self.0)
    }
}

impl Header for ContentLocation {
    const NAME: &'static str = "Content-Location";
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

impl Deref for ContentType {
    type Target = MediaType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for ContentEncoding {
    type Target = Vec<Coding>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for ContentLanguage {
    type Target = Vec<LanguageTag>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for ContentLocation {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<MediaType> for ContentType {
    fn from(t: MediaType) -> Self {
        ContentType(t)
    }
}

impl From<Vec<Coding>> for ContentEncoding {
    fn from(t: Vec<Coding>) -> Self {
        ContentEncoding(t)
    }
}

impl From<Vec<LanguageTag>> for ContentLanguage {
    fn from(t: Vec<LanguageTag>) -> Self {
        ContentLanguage(t)
    }
}

impl From<Url> for ContentLocation {
    fn from(t: Url) -> Self {
        ContentLocation(t)
    }
}
