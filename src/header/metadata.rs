use std::str;

use header::{RequestHeader, ResponseHeader, parse_value, serialize_value, parse_list1,
             serialize_list};
use header::item::{MediaType, Coding, LanguageTag, Url};

header!{
    /// `Content-Type` header, [RFC7231 Section 3.1.1.5]
    pub struct ContentType(MediaType);
    (RequestHeader ResponseHeader);
    NAME = "Content-Type";
    SENSITIVE = false;
    parse(s, _base) {
        parse_value::<MediaType>(s).map(Into::into)
    }
    serialize(self, iter) {
        serialize_value(iter, &self.0)
    }
}

header!{
    /// `Content-Encoding` header, [RFC7231 Section 3.1.2.2]
    pub struct ContentEncoding(Vec<Coding>);
    (RequestHeader ResponseHeader);
    NAME = "Content-Encoding";
    SENSITIVE = false;
    parse(s, _base) {
        parse_list1(s).map(Into::into)
    }
    serialize(self, iter) {
        serialize_list(iter, &self.0)
    }
}

header!{
    /// `Content-Language` header, [RFC7231 Section 3.1.3.2]
    pub struct ContentLanguage(Vec<LanguageTag>);
    (RequestHeader ResponseHeader);
    NAME = "Content-Language";
    SENSITIVE = false;
    parse(s, _base) {
        parse_list1(s).map(Into::into)
    }
    serialize(self, iter) {
        serialize_list(iter, &self.0)
    }
}

header!{
    /// `Content-Location` header, [RFC7231 Section 3.1.4.2]
    pub struct ContentLocation(Url);
    (RequestHeader ResponseHeader);
    NAME = "Content-Location";
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
