

/// Represents a HTTP method.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BridgeRequestMethod {
    Put,
    Post,
    Get,
    Delete,
}