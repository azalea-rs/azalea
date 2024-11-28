use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use azalea_buf::AzBuf;

#[derive(Clone, Copy, Debug, AzBuf)]
pub enum ObjectiveCriteria {
    Integer,
    Hearts,
}

impl Display for ObjectiveCriteria {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
