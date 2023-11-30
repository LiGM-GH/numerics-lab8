use std::fmt;

use error_stack::Context;

#[derive(Debug)]
pub struct SomeError;

impl fmt::Display for SomeError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("something happened")
    }
}

impl Context for SomeError {}
