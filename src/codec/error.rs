use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Exception {
    TypeError,
    InvalidStateError,
    NotSupportedError,
    InternalError,
    DecodeError,
    AbortError,
}
