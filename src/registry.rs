//! Dynamic provider registry.
//!
//! Allows registering and looking up [DatasetProvider] implementations
//! by name, supporting a plugin-style architecture.

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::error::{CoreError, Result};
use crate::providers::{AbsProvider, StatsNzProvider};
use crate::traits::DatasetProvider;

pub type SharedProvider = Arc<dyn DatasetProvider>;

#[derive(Clone, Default)]
pub struct ProviderRegistry {
    providers: BTreeMap<String, SharedProvider>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        registry.register(AbsProvider::default());
        registry.register(StatsNzProvider::default());
        registry
    }

    pub fn register<P>(&mut self, provider: P)
    where
        P: DatasetProvider + 'static,
    {
        self.providers
            .insert(provider.metadata().id, Arc::new(provider));
    }

    pub fn names(&self) -> Vec<&str> {
        self.providers.keys().map(String::as_str).collect()
    }

    pub fn get(&self, name: &str) -> Result<SharedProvider> {
        self.providers
            .get(name)
            .cloned()
            .ok_or_else(|| CoreError::ProviderNotRegistered(name.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockProvider;

    #[test]
    fn registers_and_returns_provider() {
        let mut registry = ProviderRegistry::new();
        registry.register(MockProvider);
        assert_eq!(registry.names(), vec!["mock"]);
        assert!(registry.get("mock").is_ok());
        assert!(registry.get("missing").is_err());
    }
}