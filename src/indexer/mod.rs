use algonaut_client::{
    error::{AlgorandError, BuilderError},
    indexer::v2::Client,
};

pub mod v2;

/// Indexer is the entry point to the creation of a client for the Algorand's indexer
pub struct IndexerBuilder<'a> {
    url: Option<&'a str>,
}

impl<'a> IndexerBuilder<'a> {
    /// Start the creation of a client.
    pub fn new() -> Self {
        Self::default()
    }

    /// Bind to a URL.
    pub fn bind(mut self, url: &'a str) -> Self {
        self.url = Some(url);
        self
    }

    /// Build a v2 client for Algorand's indexer.
    pub fn build_v2(self) -> Result<v2::Indexer, AlgorandError> {
        match self.url {
            Some(url) => Ok(v2::Indexer::new(Client::new(url)?)),
            None => Err(BuilderError::UnitializedUrl.into()),
        }
    }
}

impl<'a> Default for IndexerBuilder<'a> {
    fn default() -> Self {
        IndexerBuilder { url: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_client_builder() {
        let indexer = IndexerBuilder::new().bind("http://example.com").build_v2();

        assert!(indexer.ok().is_some());
    }

    #[test]
    #[should_panic(expected = "")]
    fn test_client_builder_with_no_url() {
        let _ = IndexerBuilder::new().build_v2().unwrap();
    }
}