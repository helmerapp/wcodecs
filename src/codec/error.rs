use std::fmt::Debug;

pub enum Exception {
    TypeError,
    InvalidStateError,
}

impl Debug for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exception::TypeError => f.write_str("TypeError"),
            Exception::InvalidStateError => f.write_str("InvalidStateError"),
        }
    }
}
