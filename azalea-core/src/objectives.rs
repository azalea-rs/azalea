use std::{
    fmt::{self, Display},
    str::FromStr,
};

use azalea_buf::AzBuf;

#[derive(Clone, Copy, Debug, AzBuf, PartialEq)]
pub enum ObjectiveCriteria {
    Integer,
    Hearts,
}

impl Display for ObjectiveCriteria {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectiveCriteria::Integer => write!(f, "integer"),
            ObjectiveCriteria::Hearts => write!(f, "hearts"),
        }
    }
}

impl FromStr for ObjectiveCriteria {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "integer" => Ok(ObjectiveCriteria::Integer),
            "hearts" => Ok(ObjectiveCriteria::Hearts),
            _ => Err(()),
        }
    }
}
