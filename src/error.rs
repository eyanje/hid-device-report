use std::fmt::{self, Display, Formatter};
use std::error::Error;

/// Error type when a report descriptor has some fields with ID and others without.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MissingIdError {}
impl Display for MissingIdError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        "reports should have IDs if one has an ID".fmt(fmt)
    }
}
impl Error for MissingIdError {}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TryFromIntError {}

impl Display for TryFromIntError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        "out of range integral type conversion attempted".fmt(fmt)
    }
}
impl Error for TryFromIntError {}

