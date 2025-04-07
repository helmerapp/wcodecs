#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    Unconfigured,
    Configured,
    Closed,
}
