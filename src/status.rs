use std::fmt::{self, Display};

/// The status-code element is a three-digit integer code giving the
/// result of the attempt to understand and satisfy the request.
///
/// * Source: [Hypertext Transfer Protocol (HTTP) Status Code Registry]
/// (http://www.iana.org/assignments/http-status-codes/)
/// * Revision: 2016-03-01
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Status(u16);

impl Status {
    /// 100: Continue, [RFC7231, Section 6.2.1]
    pub const CONTINUE: Status = Status(100);
    /// 101: Switching Protocols, [RFC7231, Section 6.2.2]
    pub const SWITCHING_PROTOCOLS: Status = Status(101);
    /// 102: Processing, [RFC2518]
    pub const PROCESSING: Status = Status(102);

    /// 200: OK, [RFC7231, Section 6.3.1]
    pub const OK: Status = Status(200);
    /// 201: Created, [RFC7231, Section 6.3.2]
    pub const CREATED: Status = Status(201);
    /// 202: Accepted, [RFC7231, Section 6.3.3]
    pub const ACCEPTED: Status = Status(202);
    /// 203: Non-Authoritative Information, [RFC7231, Section 6.3.4]
    pub const NON_AUTHORITATIVE_INFORMATION: Status = Status(203);
    /// 204: No Content, [RFC7231, Section 6.3.5]
    pub const NO_CONTENT: Status = Status(204);
    /// 205: Reset Content, [RFC7231, Section 6.3.6]
    pub const RESET_CONTENT: Status = Status(205);
    /// 206: Partial Content, [RFC7233, Section 4.1]
    pub const PARTIAL_CONTENT: Status = Status(206);
    /// 207: Multi-Status, [RFC4918]
    pub const MULTI_STATUS: Status = Status(207);
    /// 208: Already Reported, [RFC5842]
    pub const ALREADY_REPORTED: Status = Status(208);
    /// 226: IM Used, [RFC3229]
    pub const IM_USED: Status = Status(226);

    /// 300: Multiple Choices, [RFC7231, Section 6.4.1]
    pub const MULTIPLE_CHOICES: Status = Status(300);
    /// 301: Moved Permanently, [RFC7231, Section 6.4.2]
    pub const MOVED_PERMANENTLY: Status = Status(301);
    /// 302: Found, [RFC7231, Section 6.4.3]
    pub const FOUND: Status = Status(302);
    /// 303: See Other, [RFC7231, Section 6.4.4]
    pub const SEE_OTHER: Status = Status(303);
    /// 304: Not Modified, [RFC7232, Section 4.1]
    pub const NOT_MODIFIED: Status = Status(304);
    /// 305: Use Proxy, [RFC7231, Section 6.4.5]
    pub const USE_PROXY: Status = Status(305);
    /// 307: Temporary Redirect, [RFC7231, Section 6.4.7]
    pub const TEMPORARY_REDIRECT: Status = Status(307);
    /// 308: Permanent Redirect, [RFC7538]
    pub const PERMANENT_REDIRECT: Status = Status(308);

    /// 400: Bad Request, [RFC7231, Section 6.5.1]
    pub const BAD_REQUEST: Status = Status(400);
    /// 401: Unauthorized, [RFC7235, Section 3.1]
    pub const UNAUTHORIZED: Status = Status(401);
    /// 402: Payment Required, [RFC7231, Section 6.5.2]
    pub const PAYMENT_REQUIRED: Status = Status(402);
    /// 403: Forbidden, [RFC7231, Section 6.5.3]
    pub const FORBIDDEN: Status = Status(403);
    /// 404: Not Found, [RFC7231, Section 6.5.4]
    pub const NOT_FOUND: Status = Status(404);
    /// 405: Method Not Allowed, [RFC7231, Section 6.5.5]
    pub const METHOD_NOT_ALLOWED: Status = Status(405);
    /// 406: Not Acceptable, [RFC7231, Section 6.5.6]
    pub const NOT_ACCEPTABLE: Status = Status(406);
    /// 407: Proxy Authentication Required, [RFC7235, Section 3.2]
    pub const PROXY_AUTHENTICATION_REQUIRED: Status = Status(407);
    /// 408: Request Timeout, [RFC7231, Section 6.5.7]
    pub const REQUEST_TIMEOUT: Status = Status(408);
    /// 409: Conflict, [RFC7231, Section 6.5.8]
    pub const CONFLICT: Status = Status(409);
    /// 410: Gone, [RFC7231, Section 6.5.9]
    pub const GONE: Status = Status(410);
    /// 411: Length Required, [RFC7231, Section 6.5.10]
    pub const LENGTH_REQUIRED: Status = Status(411);
    /// 412: Precondition Failed, [RFC7232, Section 4.2]
    pub const PRECONDITION_FAILED: Status = Status(412);
    /// 413: Payload Too Large, [RFC7231, Section 6.5.11]
    pub const PAYLOAD_TOO_LARGE: Status = Status(413);
    /// 414: URI Too Long, [RFC7231, Section 6.5.12]
    pub const URI_TOO_LONG: Status = Status(414);
    /// 415: Unsupported Media Type, [RFC7231, Section 6.5.13][RFC7694, Section 3]
    pub const UNSUPPORTED_MEDIA_TYPE: Status = Status(415);
    /// 416: Range Not Satisfiable, [RFC7233, Section 4.4]
    pub const RANGE_NOT_SATISFIABLE: Status = Status(416);
    /// 417: Expectation Failed, [RFC7231, Section 6.5.14]
    pub const EXPECTATION_FAILED: Status = Status(417);
    /// 421: Misdirected Request, [RFC7540, Section 9.1.2]
    pub const MISDIRECTED_REQUEST: Status = Status(421);
    /// 422: Unprocessable Entity, [RFC4918]
    pub const UNPROCESSABLE_ENTITY: Status = Status(422);
    /// 423: Locked, [RFC4918]
    pub const LOCKED: Status = Status(423);
    /// 424: Failed Dependency, [RFC4918]
    pub const FAILED_DEPENDENCY: Status = Status(424);
    /// 426: Upgrade Required, [RFC7231, Section 6.5.15]
    pub const UPGRADE_REQUIRED: Status = Status(426);
    /// 428: Precondition Required, [RFC6585]
    pub const PRECONDITION_REQUIRED: Status = Status(428);
    /// 429: Too Many Requests, [RFC6585]
    pub const TOO_MANY_REQUESTS: Status = Status(429);
    /// 431: Request Header Fields Too Large, [RFC6585]
    pub const REQUEST_HEADER_FIELDS_TOO_LARGE: Status = Status(431);
    /// 451: Unavailable For Legal Reasons, [RFC7725]
    pub const UNAVAILABLE_FOR_LEGAL_REASONS: Status = Status(451);

    /// 500: Internal Server Error, [RFC7231, Section 6.6.1]
    pub const INTERNAL_SERVER_ERROR: Status = Status(500);
    /// 501: Not Implemented, [RFC7231, Section 6.6.2]
    pub const NOT_IMPLEMENTED: Status = Status(501);
    /// 502: Bad Gateway, [RFC7231, Section 6.6.3]
    pub const BAD_GATEWAY: Status = Status(502);
    /// 503: Service Unavailable, [RFC7231, Section 6.6.4]
    pub const SERVICE_UNAVAILABLE: Status = Status(503);
    /// 504: Gateway Timeout, [RFC7231, Section 6.6.5]
    pub const GATEWAY_TIMEOUT: Status = Status(504);
    /// 505: HTTP Version Not Supported, [RFC7231, Section 6.6.6]
    pub const HTTP_VERSION_NOT_SUPPORTED: Status = Status(505);
    /// 506: Variant Also Negotiates, [RFC2295]
    pub const VARIANT_ALSO_NEGOTIATES: Status = Status(506);
    /// 507: Insufficient Storage, [RFC4918]
    pub const INSUFFICIENT_STORAGE: Status = Status(507);
    /// 508: Loop Detected, [RFC5842]
    pub const LOOP_DETECTED: Status = Status(508);
    /// 510: Not Extended, [RFC2774]
    pub const NOT_EXTENDED: Status = Status(510);
    /// 511: Network Authentication Required, [RFC6585]
    pub const NETWORK_AUTHENTICATION_REQUIRED: Status = Status(511);

    /// Create a new status code from a numeric code.
    ///
    /// Use of this method is discouraged. For registered status
    /// codes should be created using the associated constants.
    ///
    /// # Panics
    /// Only valid status codes in the range 100 to 599 can be created,
    /// codes outside the range will panic. To construct other codes
    /// (e.g. for `XmlHttpRequest`) you must use `from_raw`.
    pub fn new(code: u16) -> Status {
        assert!(code >= 100 && code < 600,
                "valid status codes are in the range 100 to 599");
        Status(code)
    }

    /// Constructs a status code from a `u16` number.
    ///
    /// This does not check if the code is in the valid range.
    /// Normally you should use the `new` constructor.
    pub fn from_raw(code: u16) -> Status {
        Status(code)
    }

    /// Returns the status code as a `u16` number.
    pub fn to_raw(&self) -> u16 {
        self.0
    }

    /// Returns a canonical reason phrase for common status codes.
    ///
    /// If there is no canonical reason phrase for the given status
    /// `None` is returned.
    pub fn canonical_reason(&self) -> Option<&'static str> {
        Some(match self.0 {
            100 => "Continue",
            101 => "Switching Protocols",
            102 => "Processing",

            200 => "OK",
            201 => "Created",
            202 => "Accepted",
            203 => "Non-Authoritative Information",
            204 => "No Content",
            205 => "Reset Content",
            206 => "Partial Content",
            207 => "Multi-Status",
            208 => "Already Reported",
            226 => "IM Used",

            300 => "Multiple Choices",
            301 => "Moved Permanently",
            302 => "Found",
            303 => "See Other",
            304 => "Not Modified",
            305 => "Use Proxy",
            307 => "Temporary Redirect",
            308 => "Permanent Redirect",

            400 => "Bad Request",
            401 => "Unauthorized",
            402 => "Payment Required",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            406 => "Not Acceptable",
            407 => "Proxy Authentication Required",
            408 => "Request Timeout",
            409 => "Conflict",
            410 => "Gone",
            411 => "Length Required",
            412 => "Precondition Failed",
            413 => "Payload Too Large",
            414 => "URI Too Long",
            415 => "Unsupported Media Type",
            416 => "Range Not Satisfiable",
            417 => "Expectation Failed",
            421 => "Misdirected Request",
            422 => "Unprocessable Entity",
            423 => "Locked",
            424 => "Failed Dependency",
            426 => "Upgrade Required",
            428 => "Precondition Required",
            429 => "Too Many Requests",
            431 => "Request Header Fields Too Large",
            451 => "Unavailable For Legal Reasons",

            500 => "Internal Server Error",
            501 => "Not Implemented",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            504 => "Gateway Timeout",
            505 => "HTTP Version Not Supported",
            506 => "Variant Also Negotiates",
            507 => "Insufficient Storage",
            508 => "Loop Detected",
            510 => "Not Extended",
            511 => "Network Authentication Required",
            _ => return None,
        })
    }

    /// The first digit of a status code tells its status class.
    ///
    /// Unknown status codes can be handled the same as the first
    /// status code of the same class.
    pub fn class(&self) -> StatusClass {
        use self::StatusClass::*;
        match self.0 {
            100...199 => Informational,
            200...299 => Success,
            300...399 => Redirection,
            400...499 => ClientError,
            500...599 => ServerError,
            _ => NoClass,
        }
    }

    pub fn is_informational(&self) -> bool {
        self.class() == StatusClass::Informational
    }

    pub fn is_success(&self) -> bool {
        self.class() == StatusClass::Success
    }

    pub fn is_redirection(&self) -> bool {
        self.class() == StatusClass::Redirection
    }

    pub fn is_client_error(&self) -> bool {
        self.class() == StatusClass::ClientError
    }

    pub fn is_server_error(&self) -> bool {
        self.class() == StatusClass::ServerError
    }

    pub fn is_no_class(&self) -> bool {
        self.class() == StatusClass::NoClass
    }

    /// Some responses are defined as cacheable by default. [RFC7231 Section 6.1]
    ///
    /// These status codes can be reused by a cache with heuristic
    /// expiration unless otherwise indicated by the method definition or
    /// explicit cache controls.
    pub fn is_cacheable(&self) -> bool {
        self.0 == 200 || self.0 == 203 || self.0 == 204 || self.0 == 206 ||
        self.0 == 300 || self.0 == 301 || self.0 == 404 || self.0 == 405 ||
        self.0 == 410 || self.0 == 414 || self.0 == 501
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{} {}",
               self.0,
               self.canonical_reason().unwrap_or("<unknown>"))
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StatusClass {
    /// Informational codes indicate an interim response. [RFC7231, Section 6.2]
    Informational,
    /// Success codes indicates that the request was received an accepted. [RFC7231, Section 6.3]
    Success,
    /// Redirection codes indicate that the client must take further action
    /// to fulfil the request. [RFC7231, Section 6.4]
    Redirection,
    /// Client errors indicate that server believes the client has erred. [RFC7231, Section 6.5]
    ClientError,
    /// The server was not able to process the request. [RFC7231, Section 6.6]
    ServerError,
    /// Class for status codes without a defined class.
    ///
    /// These codes are invalid but may be encountered in the wild.
    NoClass,
}

impl StatusClass {
    /// Returns the fallback status code for a given class.
    ///
    /// Status codes lower than 100 and greater than 599 are
    /// have no class and are mapped to 200 OK.
    pub fn default_code(&self) -> Status {
        use self::StatusClass::*;
        match *self {
            Informational => Status::CONTINUE,
            Success | NoClass => Status::OK,
            Redirection => Status::MULTIPLE_CHOICES,
            ClientError => Status::BAD_REQUEST,
            ServerError => Status::INTERNAL_SERVER_ERROR,
        }
    }
}
