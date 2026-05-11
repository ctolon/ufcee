//! In-memory registry of adapters.
//!
//! Built once at startup, then handed to the HTTP server via `Arc<AppState>`
//! for read-only access from request handlers.

use std::collections::HashMap;
use std::sync::Arc;

use crate::{Adapter, CURRENT_CONTRACT_VERSION, DynAdapter, Error};

/// Read-only registry of adapters keyed by engine name.
#[derive(Default)]
pub struct Registry {
    adapters: HashMap<String, Arc<dyn DynAdapter>>,
}

impl Registry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an adapter.
    ///
    /// # Errors
    ///
    /// - [`Error::Adapter`] if the adapter's `contract_version` is lower than
    ///   [`CURRENT_CONTRACT_VERSION`].
    /// - [`Error::Adapter`] if an adapter with the same name is already
    ///   registered.
    pub fn register<A: Adapter>(&mut self, adapter: A) -> Result<(), Error> {
        let name = adapter.name().to_string();
        let v = adapter.contract_version();
        if v < CURRENT_CONTRACT_VERSION {
            return Err(Error::Adapter(format!(
                "adapter '{name}' contract_version {v} < core {CURRENT_CONTRACT_VERSION}"
            )));
        }
        if self.adapters.contains_key(&name) {
            return Err(Error::Adapter(format!(
                "adapter '{name}' already registered"
            )));
        }
        self.adapters.insert(name, Arc::new(adapter));
        Ok(())
    }

    /// Look up an adapter by name.
    pub fn get(&self, name: &str) -> Option<Arc<dyn DynAdapter>> {
        self.adapters.get(name).cloned()
    }

    /// Iterate registered adapter names.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.adapters.keys().map(String::as_str)
    }

    pub fn len(&self) -> usize {
        self.adapters.len()
    }

    pub fn is_empty(&self) -> bool {
        self.adapters.is_empty()
    }
}

impl std::fmt::Debug for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Registry")
            .field("names", &self.adapters.keys().collect::<Vec<_>>())
            .finish()
    }
}
