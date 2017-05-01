use std::fmt::{Display, Formatter, Result as FmtResult};
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use iron::prelude::*;
use iron::status::Status;
use iron::headers::{ContentType};
use iron::modifiers::Header;
use iron::mime::Mime;
use std::str::FromStr;
use std::error;
use postgres;
use urlencoded;
use hyper;

#[derive(Debug)]
pub enum Error {
    BadRequest,
    Unprocessable,
    NotFound,
    DbError(postgres::error::Error),
    DbConnectError(postgres::error::ConnectError),
    Unexpected,
}

impl Error {
    pub fn status(&self) -> Status {
        match *self {
            Error::BadRequest        => Status::BadRequest,
            Error::Unprocessable     => Status::UnprocessableEntity,
            Error::NotFound          => Status::NotFound,
            Error::DbError(_)        => Status::InternalServerError,
            Error::DbConnectError(_) => Status::InternalServerError,
            Error::Unexpected        => Status::InternalServerError,
        }
    }

    pub fn as_response(&self) -> Response {
        let json_type = Header(ContentType(Mime::from_str("application/json").ok().unwrap()));
        Response::with((self.status(),
                        json_type,
                        self.to_json().to_string()))
    }
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Error::BadRequest            => write!(f, "BadRequest"),
            Error::Unprocessable         => write!(f, "Unproccesable"),
            Error::NotFound              => write!(f, "NotFound"),
            Error::DbError(ref e)        => write!(f, "UnexpectedError: DBError {}", e),
            Error::DbConnectError(ref e) => write!(f, "UnexpectedError: DBConnectError {}", e),
            Error::Unexpected            => write!(f, "UnexpectedError"),
        }
    }
}

impl ToJson for Error {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("message".to_string(), self.to_string().to_json());
        Json::Object(d)
    }
}

impl From<postgres::error::Error> for Error {
    fn from(err: postgres::error::Error) -> Error {
        Error::DbError(err)
    }
}

impl From<postgres::error::ConnectError> for Error {
    fn from(err: postgres::error::ConnectError) -> Error {
        Error::DbConnectError(err)
    }
}

impl From<urlencoded::UrlDecodingError> for Error {
    fn from(_: urlencoded::UrlDecodingError) -> Error {
        Error::BadRequest
    }
}

impl From<hyper::Error> for Error {
    fn from(_: hyper::Error) -> Error {
        Error::BadRequest
    }
}

impl error::Error for Error {
    fn description(&self) -> &str { "" }
}

impl From<Error> for IronError {
    fn from(err: Error) -> IronError {
        match err {
            Error::BadRequest        => IronError::new(err, Status::BadRequest),
            Error::Unprocessable     => IronError::new(err, Status::BadRequest),
            Error::NotFound          => IronError::new(err, Status::NotFound),
            Error::DbError(_)        => IronError::new(err, Status::BadRequest),
            Error::DbConnectError(_) => IronError::new(err, Status::BadRequest),
            Error::Unexpected        => IronError::new(err, Status::BadRequest),
        }
    }
}
