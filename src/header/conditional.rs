use std::str;
use std::time::SystemTime;

use httpdate::{parse_http_date, fmt_http_date};

use header::{RequestHeader, ResponseHeader, parse_value, serialize_value,
    parse_list1_star, serialize_list_star};
use header::item::EntityTag;

header!{
    /// `ETag` header, [RFC7232 Section 2.3]
    pub struct ETag(EntityTag);
    (ResponseHeader);
    NAME = "ETag";
    SENSITIVE = false;
    parse(s, _base) {
        parse_value::<EntityTag>(s).map(Into::into)
    }
    serialize(self, iter) {
        serialize_value(iter, &self.0)
    }
}

header!{
    /// `If-Match` header, [RFC7232 Section 3.1]
    pub struct IfMatch(Vec<EntityTag>);
    (RequestHeader);
    NAME = "If-Match";
    SENSITIVE = false;
    parse(s, _base) {
        parse_list1_star(s).map(Into::into)
    }
    serialize(self, iter) {
        serialize_list_star(iter, &self.0)
    }
}

header!{
    /// `If-Modified-Since` header, [RFC7232 Section 3.3]
    pub struct IfModifiedSince(SystemTime);
    (RequestHeader);
    NAME = "If-Modified-Since";
    SENSITIVE = false;
    parse(s, _base) {
        if s.len() != 1 {
            return Err(());
        }
        let raw = str::from_utf8(&s[0]).map_err(|_| ())?;
        parse_http_date(raw).map(Into::into)
    }
    serialize(self, iter) {
        serialize_value(iter, fmt_http_date(self.0))
    }
}

header!{
    /// `If-None-Match` header, [RFC7232 Section 3.2]
    pub struct IfNoneMatch(Vec<EntityTag>);
    (RequestHeader);
    NAME = "If-None-Match";
    SENSITIVE = false;
    parse(s, _base) {
        parse_list1_star(s).map(Into::into)
    }
    serialize(self, iter) {
        serialize_list_star(iter, &self.0)
    }
}

header!{
    /// `If-Unmodified-Since` header, [RFC7232 Section 3.4]
    pub struct IfUnmodifiedSince(SystemTime);
    (RequestHeader);
    NAME = "If-Unmodified-Since";
    SENSITIVE = false;
    parse(s, _base) {
        if s.len() != 1 {
            return Err(());
        }
        let raw = str::from_utf8(&s[0]).map_err(|_| ())?;
        parse_http_date(raw).map(Into::into)
    }
    serialize(self, iter) {
        serialize_value(iter, fmt_http_date(self.0))
    }
}

header!{
    /// `Last-Modified` header, [RFC7232 Section 2.2]
    pub struct LastModified(SystemTime);
    (ResponseHeader);
    NAME = "Last-Modified";
    SENSITIVE = false;
    parse(s, _base) {
        if s.len() != 1 {
            return Err(());
        }
        let raw = str::from_utf8(&s[0]).map_err(|_| ())?;
        parse_http_date(raw).map(Into::into)
    }
    serialize(self, iter) {
        serialize_value(iter, fmt_http_date(self.0))
    }
}
