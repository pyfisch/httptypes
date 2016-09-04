use std::fmt::{self, Display};
use std::str::FromStr;

use util;
use self::Method::*;

/// The method indicates the action to be performed on the target resource.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Method {
    /// ACL, [RFC3744, Section 8.1]
    Acl,
    /// BASELINE-CONTROL, [RFC3253, Section 12.6]
    BaselineControl,
    /// BIND, [RFC5842, Section 4]
    Bind,
    /// CHECKIN, [RFC3253, Section 4.4, Section 9.4]
    Checkin,
    /// CHECKOUT, [RFC3253, Section 4.3, Section 8.8]
    Checkout,
    /// CONNECT, [RFC7231, Section 4.3.6]
    Connect,
    /// COPY, [RFC4918, Section 9.8]
    Copy,
    /// DELETE, [RFC7231, Section 4.3.5]
    Delete,
    /// GET, [RFC7231, Section 4.3.1]
    Get,
    /// HEAD, [RFC7231, Section 4.3.2]
    Head,
    /// LABEL, [RFC3253, Section 8.2]
    Label,
    /// LINK, [RFC2068, Section 19.6.1.2]
    Link,
    /// LOCK, [RFC4918, Section 9.10]
    Lock,
    /// MERGE, [RFC3253, Section 11.2]
    Merge,
    /// MKACTIVITY, [RFC3253, Section 13.5]
    Mkactivity,
    /// MKCALENDAR, [RFC4791, Section 5.3.1]
    Mkcalendar,
    /// MKCOL, [RFC4918, Section 9.3]
    Mkcol,
    /// MKREDIRECTREF, [RFC4437, Section 6]
    Mkredirectref,
    /// MKWORKSPACE, [RFC3253, Section 6.3]
    Mkworkspace,
    /// MOVE, [RFC4918, Section 9.9]
    Move,
    /// OPTIONS, [RFC7231, Section 4.3.7]
    Options,
    /// ORDERPATCH, [RFC3648, Section 7]
    Orderpatch,
    /// PATCH, [RFC5789, Section 2]
    Patch,
    /// POST, [RFC7231, Section 4.3.3]
    Post,
    /// PRI, [RFC7540, Section 3.5]
    Pri,
    /// PROPFIND, [RFC4918, Section 9.1]
    Propfind,
    /// PROPPATCH, [RFC4918, Section 9.2]
    Proppatch,
    /// PUT, [RFC7231, Section 4.3.4]
    Put,
    /// REBIND, [RFC5842, Section 6]
    Rebind,
    /// REPORT, [RFC3253, Section 3.6]
    Report,
    /// SEARCH, [RFC5323, Section 2]
    Search,
    /// TRACE, [RFC7231, Section 4.3.8]
    Trace,
    /// UNBIND, [RFC5842, Section 5]
    Unbind,
    /// UNCHECKOUT, [RFC3253, Section 4.5]
    Uncheckout,
    /// UNLINK, [RFC2068, Section 19.6.1.3]
    Unlink,
    /// UNLOCK, [RFC4918, Section 9.11]
    Unlock,
    /// UPDATE, [RFC3253, Section 7.1]
    Update,
    /// UPDATEREDIRECTREF, [RFC4437, Section 7]
    Updateredirectref,
    /// VERSION-CONTROL, [RFC3253, Section 3.5]
    VersionControl,
    /// Any other unknown method.
    ///
    /// All characters allowed in a [token] may be used.
    Unregistered(String),
}

const MAPPING: [(Method, &'static str, bool, bool); 39] =
    [(Acl, "ACL", false, true),
     (BaselineControl, "BASELINE-CONTROL", false, true),
     (Bind, "BIND", false, true),
     (Checkin, "CHECKIN", false, true),
     (Checkout, "CHECKOUT", false, true),
     (Connect, "CONNECT", false, false),
     (Copy, "COPY", false, true),
     (Delete, "DELETE", false, true),
     (Get, "GET", true, true),
     (Head, "HEAD", true, true),
     (Label, "LABEL", false, true),
     (Link, "LINK", false, true),
     (Lock, "LOCK", false, false),
     (Merge, "MERGE", false, true),
     (Mkactivity, "MKACTIVITY", false, true),
     (Mkcalendar, "MKCALENDAR", false, true),
     (Mkcol, "MKCOL", false, true),
     (Mkredirectref, "MKREDIRECTREF", false, true),
     (Mkworkspace, "MKWORKSPACE", false, true),
     (Move, "MOVE", false, true),
     (Options, "OPTIONS", true, true),
     (Orderpatch, "ORDERPATCH", false, true),
     (Patch, "PATCH", false, false),
     (Post, "POST", false, false),
     (Pri, "PRI", true, true),
     (Propfind, "PROPFIND", true, true),
     (Proppatch, "PROPPATCH", false, true),
     (Put, "PUT", false, true),
     (Rebind, "REBIND", false, true),
     (Report, "REPORT", true, true),
     (Search, "SEARCH", true, true),
     (Trace, "TRACE", true, true),
     (Unbind, "UNBIND", false, true),
     (Uncheckout, "UNCHECKOUT", false, true),
     (Unlink, "UNLINK", false, true),
     (Unlock, "UNLOCK", false, true),
     (Update, "UPDATE", false, true),
     (Updateredirectref, "UPDATEREDIRECTREF", false, true),
     (VersionControl, "VERSION-CONTROL", false, true)];

impl Method {
    /// Safe methods are essentially read-only.
    pub fn is_safe(&self) -> bool {
        if matches!(self, &Unregistered(_)) {
            return false;
        }
        MAPPING.iter()
            .find(|&&(ref method, _, _, _)| method == self)
            .map(|&(_, _, b, _)| b)
            .unwrap()
    }

    /// Idempotent methods may be called multiple times on the same
    /// resource but do not change it after the first call.
    pub fn is_idempotent(&self) -> bool {
        if matches!(self, &Unregistered(_)) {
            return false;
        }
        MAPPING.iter()
            .find(|&&(ref method, _, _, _)| method == self)
            .map(|&(_, _, _, b)| b)
            .unwrap()
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Unregistered(ref s) = *self {
            return f.write_str(s);
        }
        f.write_str(MAPPING.iter()
            .find(|&&(ref method, _, _, _)| method == self)
            .map(|&(_, s, _, _)| s)
            .unwrap())
    }
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Method, ()> {
        MAPPING.iter()
            .find(|&&(_, name, _, _)| s == name)
            .map(|&(ref method, _, _, _)| Ok(method.clone()))
            .unwrap_or_else(|| if util::is_token(s) {
                Ok(Unregistered(s.to_owned()))
            } else {
                Err(())
            })
    }
}
