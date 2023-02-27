use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use warp::reject::Reject;

#[derive(Debug)]
pub(crate) enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => {
                write!(f, "Cannot parse parameter: {}", err)
            },
            Error::MissingParameters => {
                write!(f, "Missing parameters")
            }
        }
    }
}

impl Reject for Error {}