use std::str;

use header::{RequestHeader, ResponseHeader, parse_value, serialize_value, parse_list0,
             serialize_list};
use header::item::Url;
use Method;

header!{
    /// `From` header, [RFC7231 Section 5.5.1]
    pub struct From(String);
    (RequestHeader);
    NAME = "From";
    SENSITIVE = false;
    parse(s, _base) {
        parse_value::<String>(s).map(Into::into)
    }
    serialize(self, iter) {
        serialize_value(iter, &self.0)
    }
}

header!{
    /// `Referer` header, [RFC7231 Section 5.5.2]
    pub struct Referer(Url);
    (RequestHeader);
    NAME = "Referer";
    SENSITIVE = false;
    parse(s, _base) {
        parse_value::<Url>(s).map(Into::into)
    }
    serialize(self, iter) {
        serialize_value(iter, &self.0)
    }
}

header!{
    /// `User-Agent` header, [RFC7231 Section 5.5.3]
    ///
    /// The value is decoded as UTF-8. Invalid bytes are replaced
    /// with U+FFFD REPLACEMENT CHARACTER.
    pub struct UserAgent(String);
    (RequestHeader);
    NAME = "User-Agent";
    SENSITIVE = false;
    parse(s, _base) {
        parse_value::<String>(s).map(Into::into)
    }
    serialize(self, iter) {
        serialize_value(iter, &self.0)
    }
}

header!{
    /// `Allow` header, [RFC7231 Section 7.4.1]
    pub struct Allow(Vec<Method>);
    (ResponseHeader);
    NAME = "Allow";
    SENSITIVE = false;
    parse(s, _base) {
        parse_list0::<Method>(s).map(Into::into)
    }
    serialize(self, iter) {
        serialize_list(iter, &self.0)
    }
}

header!{
    /// `Server` header, [RFC7231 Section 7.4.2]
    ///
    /// The value is decoded as UTF-8. Invalid bytes are replaced
    /// with U+FFFD REPLACEMENT CHARACTER.
    pub struct Server(String);
    (ResponseHeader);
    NAME = "SERVER";
    SENSITIVE = false;
    parse(s, _base) {
        parse_value::<String>(s).map(Into::into)
    }
    serialize(self, iter) {
        serialize_value(iter, &self.0)
    }
}
