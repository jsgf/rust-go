use std::result;
use nom::IResult;

mod parser;
pub mod node;
pub mod property;

pub use self::property::Property;
pub use self::node::Node;
pub use self::parser::collection;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Error {
    ParseError(String),
    ValueError,
}

pub fn parser(txt: &[u8]) -> Result<Vec<Node>> {
    match collection(txt) {
        IResult::Done(_, v) => Ok(v),
        e@IResult::Error(_) | e@IResult::Incomplete(..) =>
            Err(Error::ParseError(format!("{:?}", e))),

    }
}
