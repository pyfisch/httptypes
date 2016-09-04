use std::io::{self, Write};
use std::ops::Deref;

use header::{Header, RequestHeader, parse_list0, parse_list1, serialize_list};
use header::item::{Charset, Coding, Language, MediaType, Url, Quality};

#[derive(Clone, Debug)]
pub struct Accept(Vec<Quality<MediaType>>);

#[derive(Clone, Debug)]
pub struct AcceptCharset(Vec<Quality<Charset>>);

#[derive(Clone, Debug)]
pub struct AcceptEncoding(Vec<Quality<Coding>>);

#[derive(Clone, Debug)]
pub struct AcceptLanguage(Vec<Quality<Language>>);

impl RequestHeader for Accept {}
impl RequestHeader for AcceptCharset {}
impl RequestHeader for AcceptEncoding {}
impl RequestHeader for AcceptLanguage {}

impl Header for Accept {
    const NAME: &'static str = "Accept";
    const SENSITIVE: bool = false;

    fn parse(s: &[Vec<u8>], _base: Url) -> Result<Self, ()> {
        parse_list0(s).map(Into::into)
    }

    fn serialize<I: Iterator<Item=W>, W: Write>(&self, iter: I) -> io::Result<()> {
        serialize_list(iter, &self.0)
    }
}

impl Header for AcceptCharset {
    const NAME: &'static str = "Accept-Charset";
    const SENSITIVE: bool = false;

    fn parse(s: &[Vec<u8>], _base: Url) -> Result<Self, ()> {
        parse_list1(s).map(Into::into)
    }

    fn serialize<I: Iterator<Item=W>, W: Write>(&self, iter: I) -> io::Result<()> {
        serialize_list(iter, &self.0)
    }
}

impl Header for AcceptEncoding {
    const NAME: &'static str = "Accept-Encoding";
    const SENSITIVE: bool = false;

    fn parse(s: &[Vec<u8>], _base: Url) -> Result<Self, ()> {
        parse_list0(s).map(Into::into)
    }

    fn serialize<I: Iterator<Item=W>, W: Write>(&self, iter: I) -> io::Result<()> {
        serialize_list(iter, &self.0)
    }
}

impl Header for AcceptLanguage {
    const NAME: &'static str = "Accept-Language";
    const SENSITIVE: bool = false;

    fn parse(s: &[Vec<u8>], _base: Url) -> Result<Self, ()> {
        parse_list1(s).map(Into::into)
    }

    fn serialize<I: Iterator<Item=W>, W: Write>(&self, iter: I) -> io::Result<()> {
        serialize_list(iter, &self.0)
    }
}

impl Deref for Accept {
    type Target = Vec<Quality<MediaType>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for AcceptCharset {
    type Target = Vec<Quality<Charset>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for AcceptEncoding {
    type Target = Vec<Quality<Coding>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for AcceptLanguage {
    type Target = Vec<Quality<Language>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<Quality<MediaType>>> for Accept {
    fn from(t: Vec<Quality<MediaType>>) -> Self {
        Accept(t)
    }
}

impl From<Vec<Quality<Charset>>> for AcceptCharset {
    fn from(t: Vec<Quality<Charset>>) -> Self {
        AcceptCharset(t)
    }
}

impl From<Vec<Quality<Coding>>> for AcceptEncoding {
    fn from(t: Vec<Quality<Coding>>) -> Self {
        AcceptEncoding(t)
    }
}

impl From<Vec<Quality<Language>>> for AcceptLanguage {
    fn from(t: Vec<Quality<Language>>) -> Self {
        AcceptLanguage(t)
    }
}
