use std::{collections::HashMap, fs::read_to_string};

use super::{JsonObject, JsonArray};

#[derive(Debug)]
pub struct Request {
    pub request_type: RequestType,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<BodyContents>,
}
impl Request {
    pub fn has_data(&self) -> bool {
        return self.body.is_some();
    }
}

pub struct Response {
    data: Vec<u8>,
    status: ResponseStatusCode,
}
impl Response {
    pub fn new() -> Response {
        Response {
            data: Vec::new(),
            status: ResponseStatusCode::Ok,
        }
    }
    pub fn header(&self) -> Vec<u8> {
        let mut output = String::from("HTTP/1.1 ");
        output += &self.status.http_string();
        output += "\r\n\r\n";
        return output.into_bytes();
    }

    pub fn set_status(&mut self, status: ResponseStatusCode) {
        self.status = status;
    }

    // Functions for setting data
    pub fn send_string<S: AsRef<str>>(&mut self, s: S) {
        self.data = s.as_ref().as_bytes().to_vec();
    }

    // Get byts out
    pub fn bytes(self) -> Vec<u8> {
        return self.data;
    }
}

pub enum ResponseStatusCode {
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    TemporaryRedirect,
    PermanentRedirect,
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    UriTooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    TooEarly,
    PreconditionRequired,
    TooManyRequests,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HttpVersionNotSupported,
}
impl ResponseStatusCode {
    pub fn code(&self) -> i32 {
        match self {
            ResponseStatusCode::Ok => 200,
            ResponseStatusCode::Created => 201,
            ResponseStatusCode::Accepted => 202,
            ResponseStatusCode::NonAuthoritativeInformation => 203,
            ResponseStatusCode::NoContent => 204,
            ResponseStatusCode::ResetContent => 205,
            ResponseStatusCode::PartialContent => 206,
            ResponseStatusCode::MultipleChoices => 300,
            ResponseStatusCode::MovedPermanently => 301,
            ResponseStatusCode::Found => 302,
            ResponseStatusCode::SeeOther => 303,
            ResponseStatusCode::NotModified => 304,
            ResponseStatusCode::TemporaryRedirect => 307,
            ResponseStatusCode::PermanentRedirect => 308,
            ResponseStatusCode::BadRequest => 400,
            ResponseStatusCode::Unauthorized => 401,
            ResponseStatusCode::PaymentRequired => 402,
            ResponseStatusCode::Forbidden => 403,
            ResponseStatusCode::NotFound => 404,
            ResponseStatusCode::MethodNotAllowed => 405,
            ResponseStatusCode::NotAcceptable => 406,
            ResponseStatusCode::RequestTimeout => 408,
            ResponseStatusCode::Conflict => 409,
            ResponseStatusCode::Gone => 410,
            ResponseStatusCode::LengthRequired => 411,
            ResponseStatusCode::PreconditionFailed => 412,
            ResponseStatusCode::PayloadTooLarge => 413,
            ResponseStatusCode::UriTooLong => 414,
            ResponseStatusCode::UnsupportedMediaType => 415,
            ResponseStatusCode::RangeNotSatisfiable => 416,
            ResponseStatusCode::ExpectationFailed => 417,
            ResponseStatusCode::ImATeapot => 418,
            ResponseStatusCode::TooEarly => 425,
            ResponseStatusCode::PreconditionRequired => 428,
            ResponseStatusCode::TooManyRequests => 429,
            ResponseStatusCode::InternalServerError => 500,
            ResponseStatusCode::NotImplemented => 501,
            ResponseStatusCode::BadGateway => 502,
            ResponseStatusCode::ServiceUnavailable => 503,
            ResponseStatusCode::GatewayTimeout => 504,
            ResponseStatusCode::HttpVersionNotSupported => 505,
        }
    }
    pub fn http_string(&self) -> String {
        match self {
            ResponseStatusCode::Ok => "200 OK".to_string(),
            ResponseStatusCode::Created => "201 Created".to_string(),
            ResponseStatusCode::Accepted => "202 Accepted".to_string(),
            ResponseStatusCode::NonAuthoritativeInformation => {
                "203 Non-Authoritative Information".to_string()
            }
            ResponseStatusCode::NoContent => "204 No Content".to_string(),
            ResponseStatusCode::ResetContent => "205 Reset Content".to_string(),
            ResponseStatusCode::PartialContent => "206 Partial Content".to_string(),
            ResponseStatusCode::MultipleChoices => "300 Multiple Choices".to_string(),
            ResponseStatusCode::MovedPermanently => "301 Moved Permanently".to_string(),
            ResponseStatusCode::Found => "302 Found".to_string(),
            ResponseStatusCode::SeeOther => "303 See Other".to_string(),
            ResponseStatusCode::NotModified => "304 Not Modified".to_string(),
            ResponseStatusCode::TemporaryRedirect => "307 Temporary Redirect".to_string(),
            ResponseStatusCode::PermanentRedirect => "308 Permanent Redirect".to_string(),
            ResponseStatusCode::BadRequest => "400 Bad Request".to_string(),
            ResponseStatusCode::Unauthorized => "401 Unauthorized".to_string(),
            ResponseStatusCode::PaymentRequired => "402 Payment Required".to_string(),
            ResponseStatusCode::Forbidden => "403 Forbidden".to_string(),
            ResponseStatusCode::NotFound => "404 Not Found".to_string(),
            ResponseStatusCode::MethodNotAllowed => "405 Method Not Allowed".to_string(),
            ResponseStatusCode::NotAcceptable => "406 Not Allowed".to_string(),
            ResponseStatusCode::RequestTimeout => "408 Request Timeout".to_string(),
            ResponseStatusCode::Conflict => "409 Conflict".to_string(),
            ResponseStatusCode::Gone => "410 Gone".to_string(),
            ResponseStatusCode::LengthRequired => "411 Length Required".to_string(),
            ResponseStatusCode::PreconditionFailed => "412 Precondition Failed".to_string(),
            ResponseStatusCode::PayloadTooLarge => "413 Payload Too Large".to_string(),
            ResponseStatusCode::UriTooLong => "414 URI Too Long".to_string(),
            ResponseStatusCode::UnsupportedMediaType => "415 Unsupported Media Type".to_string(),
            ResponseStatusCode::RangeNotSatisfiable => "416 Range Not Satisfiable".to_string(),
            ResponseStatusCode::ExpectationFailed => "417 Expectation Failed".to_string(),
            ResponseStatusCode::ImATeapot => "418 I'm a teapot".to_string(),
            ResponseStatusCode::TooEarly => "425 Too Early".to_string(),
            ResponseStatusCode::PreconditionRequired => "428 Precondition Required".to_string(),
            ResponseStatusCode::TooManyRequests => "429 Too Many Requests".to_string(),
            ResponseStatusCode::InternalServerError => "500 Internal Server Error".to_string(),
            ResponseStatusCode::NotImplemented => "501 Not Implemented".to_string(),
            ResponseStatusCode::BadGateway => "502 Bad Gateway".to_string(),
            ResponseStatusCode::ServiceUnavailable => "503 Service Unavailable".to_string(),
            ResponseStatusCode::GatewayTimeout => "504 Gateway Timeout".to_string(),
            ResponseStatusCode::HttpVersionNotSupported => {
                "505 HTTP Version Not Supported".to_string()
            }
        }
    }
}

#[derive(Debug)]
pub enum BodyContents {
    Binary(Vec<u8>),
    JsonObject(JsonObject),
    JsonArray(JsonArray),
    PlainText(String),
    None,
}
impl BodyContents {
    const TYPE_JSON: &str = "application/json";
    const TYPE_OCTET_STREAM: &str = "application/octet-stream";
    const TYPE_LD_JSON: &str = "application/ld+json";
    const TYPE_PLAIN_TEXT: &str = "text/plain";

    pub fn type_from_mime(mime: &str, data: Vec<u8>) -> BodyContents {
        match mime {
            BodyContents::TYPE_OCTET_STREAM => BodyContents::Binary(data),
            BodyContents::TYPE_JSON | BodyContents::TYPE_LD_JSON => {
                let contents_string = String::from_utf8(data).unwrap();
                if contents_string.chars().next().unwrap() == '[' {
                    BodyContents::JsonArray(JsonArray::from_string(contents_string))
                } else {
                    BodyContents::JsonObject(JsonObject::from_string(contents_string))
                }
            }
            BodyContents::TYPE_PLAIN_TEXT => {
                BodyContents::PlainText(String::from_utf8(data).unwrap())
            }
            _ => BodyContents::Binary(data),
        }
    }

    pub fn as_json_object(&self) -> Option<&JsonObject> {
        match self {
            BodyContents::JsonObject(j) => Some(j),
            _ => None
        }
    }    
    pub fn as_json_array(&self) -> Option<&JsonArray> {
        match self {
            BodyContents::JsonArray(j) => Some(j),
            _ => None
        }
    }
}

#[derive(Debug)]
pub enum RequestType {
    Get,
    Post,
    Put,
    Delete,
    Any,
}

impl RequestType {
    const GET_TYPE: &str = "GET";
    const POST_TYPE: &str = "POST";
    const PUT_TYPE: &str = "PUT";
    const DELETE_TYPE: &str = "DELETE";
    const ANY_TYPE: &str = "ANY";

    pub fn type_for_method(method: &str) -> RequestType {
        match method {
            RequestType::GET_TYPE => RequestType::Get,
            RequestType::POST_TYPE => RequestType::Post,
            RequestType::PUT_TYPE => RequestType::Put,
            RequestType::DELETE_TYPE => RequestType::Delete,
            _ => RequestType::Any,
        }
    }

    pub fn is_any(&self) -> bool {
        match self {
            RequestType::Get => false,
            RequestType::Post => false,
            RequestType::Put => false,
            RequestType::Delete => false,
            RequestType::Any => true,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            RequestType::Get => RequestType::GET_TYPE.to_string(),
            RequestType::Post => RequestType::POST_TYPE.to_string(),
            RequestType::Put => RequestType::PUT_TYPE.to_string(),
            RequestType::Delete => RequestType::DELETE_TYPE.to_string(),
            RequestType::Any => RequestType::ANY_TYPE.to_string(),
        }
    }
}
