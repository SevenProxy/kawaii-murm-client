//! MIP-04: initial event kinds.

/// Author profile metadata.
pub const PROFILE: u32 = 0;

/// Public post or article.
pub const POST: u32 = 1;

/// Reply to a post or another comment.
pub const COMMENT: u32 = 2;

/// Reaction to an event.
pub const REACTION: u32 = 3;

/// Returns the kind name defined by MIP-04, if the kind is known.
pub fn name(kind: u32) -> Option<&'static str> {
    match kind {
        PROFILE => Some("profile"),
        POST => Some("post"),
        COMMENT => Some("comment"),
        REACTION => Some("reaction"),
        _ => None,
    }
}
