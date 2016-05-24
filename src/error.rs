use std::fmt::{Display, Formatter, Result as FmtResult};
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use iron::prelude::*;
use iron::status::Status;
use iron::headers::{ContentType};
use iron::modifiers::Header;
use iron::mime::Mime;
use std::str::FromStr;
use postgres;

#[derive(Debug)]
pub enum Error {
    BadRequest,
    Unprocessable,
    NotFound,
    DbError(postgres::error::Error),
    Unexpected,
}

impl Error {
    pub fn status(&self) -> Status {
        match *self {
            Error::BadRequest    => Status::BadRequest,
            Error::Unprocessable => Status::UnprocessableEntity,
            Error::NotFound      => Status::NotFound,
            Error::DbError(_)    => Status::InternalServerError,
            Error::Unexpected    => Status::InternalServerError,
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
            Error::BadRequest     => write!(f, "BadRequest"),
            Error::Unprocessable  => write!(f, "Unproccesable"),
            Error::NotFound       => write!(f, "NotFound"),
            Error::DbError(ref e) => write!(f, "UnexpectedError: DBError {}", e),
            Error::Unexpected     => write!(f, "UnexpectedError"),
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
