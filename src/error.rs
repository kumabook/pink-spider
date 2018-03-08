use std::fmt::{Display, Formatter, Result as FmtResult};
use std::num::ParseIntError;
use iron::prelude::*;
use iron::status::Status;
use iron::headers::{ContentType};
use iron::modifiers::Header;
use iron::mime::Mime;
use serde_json;
use serde::Serialize;
use serde::Serializer;
use std::str::FromStr;
use std::error;
use postgres;
use reqwest;
use url;
use params::ParamsError;

#[derive(Debug)]
pub enum Error {
    BadRequest,
    Unprocessable,
    NotFound,
    DbError(postgres::error::DbError),
    DbConnectError(postgres::error::Error),
    UrlParseError(url::ParseError),
    Unexpected,
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self {
            Error::BadRequest        => serializer.serialize_str("BadRequest"),
            Error::Unprocessable     => serializer.serialize_str("Unprocessable"),
            Error::NotFound          => serializer.serialize_str("NotFound"),
            Error::DbError(_)        => serializer.serialize_str("DbError"),
            Error::DbConnectError(_) => serializer.serialize_str("DbConnectError"),
            Error::UrlParseError(_)  => serializer.serialize_str("UrlParseError"),
            Error::Unexpected        => serializer.serialize_str("Unexpected"),
        }
    }
}

impl Error {
    pub fn status(&self) -> Status {
        match *self {
            Error::BadRequest        => Status::BadRequest,
            Error::Unprocessable     => Status::UnprocessableEntity,
            Error::NotFound          => Status::NotFound,
            Error::DbError(_)        => Status::InternalServerError,
            Error::DbConnectError(_) => Status::InternalServerError,
            Error::UrlParseError(_)  => Status::InternalServerError,
            Error::Unexpected        => Status::InternalServerError,
        }
    }

    pub fn as_response(&self) -> Response {
        let json_type = Header(ContentType(Mime::from_str("application/json").ok().unwrap()));
        Response::with((self.status(), json_type, serde_json::to_string(self).unwrap()))
    }
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Error::BadRequest            => write!(f, "BadRequest"),
            Error::Unprocessable         => write!(f, "Unproccesable"),
            Error::NotFound              => write!(f, "NotFound"),
            Error::DbError(ref e)        => write!(f, "DBError {}", e),
            Error::DbConnectError(ref e) => write!(f, "DBConnectError {}", e),
            Error::UrlParseError(ref e)  => write!(f, "UrlParseError:  {}", e),
            Error::Unexpected            => write!(f, "UnexpectedError"),
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::UrlParseError(err)
    }
}

impl From<postgres::error::DbError> for Error {
    fn from(err: postgres::error::DbError) -> Error {
        Error::DbError(err)
    }
}

impl From<postgres::error::Error> for Error {
    fn from(err: postgres::error::Error) -> Error {
        Error::DbConnectError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Error {
        Error::BadRequest
    }
}

impl error::Error for Error {
    fn description(&self) -> &str { "" }
}

impl From<ParamsError> for Error {
    fn from(_: ParamsError) -> Error {
        Error::BadRequest
    }
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Error {
        Error::BadRequest
    }
}

impl From<Error> for IronError {
    fn from(err: Error) -> IronError {
        match err {
            Error::BadRequest        => IronError::new(err, Status::BadRequest),
            Error::Unprocessable     => IronError::new(err, Status::BadRequest),
            Error::NotFound          => IronError::new(err, Status::NotFound),
            Error::DbError(_)        => IronError::new(err, Status::InternalServerError),
            Error::DbConnectError(_) => IronError::new(err, Status::InternalServerError),
            Error::UrlParseError(_)  => IronError::new(err, Status::InternalServerError),
            Error::Unexpected        => IronError::new(err, Status::InternalServerError),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Error {
        Error::Unexpected
    }
}
