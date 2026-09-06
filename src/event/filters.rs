use serde::{Deserialize, Serialize};

/// MIP-03 event filter.
///
/// Within one filter, different fields are combined with `AND`; values inside
/// `ids`, `authors`, and `kinds` are combined with `OR`; tags are `AND`.
/// Multiple filters in a request are combined with `OR`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EventFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinds: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<u64>,
}

impl EventFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ids(mut self, ids: Vec<String>) -> Self {
        self.ids = Some(ids);
        self
    }

    pub fn authors(mut self, authors: Vec<String>) -> Self {
        self.authors = Some(authors);
        self
    }

    pub fn kinds(mut self, kinds: Vec<u32>) -> Self {
        self.kinds = Some(kinds);
        self
    }

    pub fn tags(mut self, tags: Vec<Vec<String>>) -> Self {
        self.tags = Some(tags);
        self
    }

    pub fn since(mut self, since: u64) -> Self {
        self.since = Some(since);
        self
    }

    pub fn until(mut self, until: u64) -> Self {
        self.until = Some(until);
        self
    }
}
