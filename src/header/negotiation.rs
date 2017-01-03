use header::{RequestHeader, parse_list0, parse_list1, serialize_list};
use header::item::{Charset, Coding, LanguageTag, MediaType, Url, Quality};

header!{
    /// `Accept` header, [RFC7231 Section 5.3.2]
    pub struct Accept(Vec<Quality<MediaType>>);
    (RequestHeader);
    NAME = "Accept";
    SENSITIVE = false;
    parse(s, _base) {
        parse_list0(s).map(Into::into)
    }
    serialize(self, iter) {
        serialize_list(iter, &self.0)
    }
}

header!{
    /// `Accept-Charset` header, [RFC7231 Section 5.3.3]
    pub struct AcceptCharset(Vec<Quality<Charset>>);
    (RequestHeader);
    NAME = "Accept-Charset";
    SENSITIVE = false;
    parse(s, _base) {
        parse_list1(s).map(Into::into)
    }
    serialize(self, iter) {
        serialize_list(iter, &self.0)
    }
}

header!{
    /// `Accept-Encoding` header, [RFC7231 Section 5.3.4]
    pub struct AcceptEncoding(Vec<Quality<Coding>>);
    (RequestHeader);
    NAME = "Accept-Encoding";
    SENSITIVE = false;
    parse(s, _base) {
        parse_list0(s).map(Into::into)
    }
    serialize(self, iter) {
        serialize_list(iter, &self.0)
    }
}

header!{
    /// `Accept-Language` header, [RFC7231 Section 5.3.5]
    pub struct AcceptLanguage(Vec<Quality<LanguageTag>>);
    (RequestHeader);
    NAME = "Accept-Language";
    SENSITIVE = false;

    parse(s, _base) {
        parse_list1(s).map(Into::into)
    }
    serialize(self, iter) {
        serialize_list(iter, &self.0)
    }
}
