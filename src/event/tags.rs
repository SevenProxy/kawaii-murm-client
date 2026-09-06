//! Helpers for building MIP-04 tags.
//!
//! A tag is an array of at least two strings: a name and a value (MIP-01).

/// Builds a generic tag with a name and a value.
pub fn tag(name: &str, value: &str) -> Vec<String> {
    vec![name.to_string(), value.to_string()]
}

/// `["root", "<root-post-event-id>"]` — the post that owns the thread.
pub fn root(post_id: &str) -> Vec<String> {
    tag("root", post_id)
}

/// `["parent", "<parent-comment-event-id>"]` — the event being replied to.
pub fn parent(comment_id: &str) -> Vec<String> {
    tag("parent", comment_id)
}

/// `["target", "<target-event-id>"]` — the event a reaction points to.
pub fn target(event_id: &str) -> Vec<String> {
    tag("target", event_id)
}

/// `["topic", "<topic>"]` — indexable topic metadata.
pub fn topic(name: &str) -> Vec<String> {
    tag("topic", name)
}

/// `["lang", "<code>"]` — language metadata.
pub fn lang(code: &str) -> Vec<String> {
    tag("lang", code)
}

/// Finds the value of the first tag with the given name.
pub fn find<'a>(tags: &'a [Vec<String>], name: &str) -> Option<&'a str> {
    tags.iter()
        .find(|tag| tag.first().map(String::as_str) == Some(name))
        .and_then(|tag| tag.get(1))
        .map(String::as_str)
}
