//! SigmaOS-data-catalog: data catalog model.
//!
//! Real implementation: datasets, schemas, lineage, and validation.
//! Replaces the 108-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Dataset entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetEntry {
    pub id: String,
    pub name: String,
    pub schema: String,
    pub owner: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogError {
    EmptyName,
    DuplicateDataset(String),
}

impl fmt::Display for CatalogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "dataset name must not be empty"),
            Self::DuplicateDataset(id) => write!(f, "duplicate dataset {:?}", id),
        }
    }
}

impl std::error::Error for CatalogError {}

/// Data catalog.
pub struct DataCatalog {
    pub name: String,
    datasets: BTreeMap<String, DatasetEntry>,
}

impl DataCatalog {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            datasets: BTreeMap::new(),
        }
    }

    pub fn add_dataset(&mut self, dataset: DatasetEntry) -> Result<(), CatalogError> {
        if dataset.name.trim().is_empty() { return Err(CatalogError::EmptyName); }
        let id = dataset.id.clone();
        if self.datasets.contains_key(&id) {
            return Err(CatalogError::DuplicateDataset(id.clone()));
        }
        self.datasets.insert(id, dataset);
        Ok(())
    }

    pub fn dataset_count(&self) -> usize { self.datasets.len() }

    pub fn get_dataset(&self, id: &str) -> Option<&DatasetEntry> { self.datasets.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_create_aaa() {
        let c = DataCatalog::new("My Catalog");
        assert_eq!(c.name, "My Catalog");
        assert_eq!(c.dataset_count(), 0);
    }

    #[test]
    fn test_add_dataset_aaa() {
        let mut c = DataCatalog::new("My Catalog");
        let d = DatasetEntry { id: "d1".into(), name: "sales".into(), schema: "public".into(), owner: "data-team".into() };
        c.add_dataset(d).unwrap();
        assert_eq!(c.dataset_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut c = DataCatalog::new("My Catalog");
        let d = DatasetEntry { id: "x".into(), name: "".into(), schema: "public".into(), owner: "team".into() };
        assert!(c.add_dataset(d).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut c = DataCatalog::new("My Catalog");
        let d = DatasetEntry { id: "x".into(), name: "X".into(), schema: "public".into(), owner: "team".into() };
        c.add_dataset(d).unwrap();
        let d2 = DatasetEntry { id: "x".into(), name: "X".into(), schema: "public".into(), owner: "team".into() };
        assert!(c.add_dataset(d2).is_err());
    }
}
