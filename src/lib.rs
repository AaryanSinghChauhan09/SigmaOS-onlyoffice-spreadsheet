//! SigmaOS-onlyoffice-spreadsheet: ONLYOFFICE spreadsheet editor model.

use std::collections::BTreeMap;
use std::fmt;

/// Primary domain entity for ONLYOFFICE spreadsheet editor model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OoSpreadsheet {
    pub id: String,
    pub name: String,
    pub sheet_count: usize,
    pub format: String,
    pub enabled: bool,
}

/// Sheet domain entity for ONLYOFFICE spreadsheet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OoSheet {
    pub id: String,
    pub title: String,
    pub cell_count: usize,
}

/// Errors for the ONLYOFFICE spreadsheet editor model domain.
#[derive(Debug, PartialEq, Eq)]
pub enum OnlyofficeSpreadsheetError {
    EmptyName,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for OnlyofficeSpreadsheetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for OnlyofficeSpreadsheetError {}

/// Registry and manager for ONLYOFFICE spreadsheet editor model.
pub struct OnlyofficeSpreadsheetManager {
    pub name: String,
    items: BTreeMap<String, OoSpreadsheet>,
    sheets: BTreeMap<String, OoSheet>,
}

impl OnlyofficeSpreadsheetManager {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: BTreeMap::new(),
            sheets: BTreeMap::new(),
        }
    }

    pub fn add_item(&mut self, item: OoSpreadsheet) -> Result<(), OnlyofficeSpreadsheetError> {
        if item.name.is_empty() {
            return Err(OnlyofficeSpreadsheetError::EmptyName);
        }
        if self.items.contains_key(&item.id) {
            return Err(OnlyofficeSpreadsheetError::Duplicate(item.id.clone()));
        }
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    pub fn get_item(&self, id: &str) -> Option<&OoSpreadsheet> {
        self.items.get(id)
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn add_sheet(&mut self, sheet: OoSheet) {
        self.sheets.insert(sheet.id.clone(), sheet);
    }

    pub fn sheet_count(&self) -> usize {
        self.sheets.len()
    }

    pub fn set_enabled(
        &mut self,
        id: &str,
        enabled: bool,
    ) -> Result<(), OnlyofficeSpreadsheetError> {
        self.items
            .get_mut(id)
            .map(|i| i.enabled = enabled)
            .ok_or_else(|| OnlyofficeSpreadsheetError::NotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_item(id: &str, name: &str) -> OoSpreadsheet {
        OoSpreadsheet {
            id: id.to_string(),
            name: name.to_string(),
            sheet_count: 3,
            format: "xlsx".to_string(),
            enabled: true,
        }
    }

    fn make_sheet(id: &str) -> OoSheet {
        OoSheet {
            id: id.to_string(),
            title: format!("Sheet {id}"),
            cell_count: 100,
        }
    }

    #[test]
    fn test_create_aaa() {
        let mgr = OnlyofficeSpreadsheetManager::new("test");
        assert_eq!(mgr.name, "test");
        assert_eq!(mgr.count(), 0);
    }

    #[test]
    fn test_add_item_aaa() {
        let mut mgr = OnlyofficeSpreadsheetManager::new("test");
        assert!(mgr.add_item(make_item("sp1", "Finance 2026")).is_ok());
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut mgr = OnlyofficeSpreadsheetManager::new("test");
        assert_eq!(
            mgr.add_item(make_item("sp1", "")),
            Err(OnlyofficeSpreadsheetError::EmptyName)
        );
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut mgr = OnlyofficeSpreadsheetManager::new("test");
        mgr.add_item(make_item("sp1", "First")).unwrap();
        assert_eq!(
            mgr.add_item(make_item("sp1", "Dup")),
            Err(OnlyofficeSpreadsheetError::Duplicate("sp1".to_string()))
        );
    }

    #[test]
    fn test_sheet_addition_aaa() {
        let mut mgr = OnlyofficeSpreadsheetManager::new("test");
        mgr.add_sheet(make_sheet("sh1"));
        assert_eq!(mgr.sheet_count(), 1);
    }

    #[test]
    fn test_get_item_aaa() {
        let mut mgr = OnlyofficeSpreadsheetManager::new("test");
        mgr.add_item(make_item("sp1", "First")).unwrap();
        assert!(mgr.get_item("sp1").is_some());
        assert!(mgr.get_item("none").is_none());
    }

    #[test]
    fn test_set_enabled_aaa() {
        let mut mgr = OnlyofficeSpreadsheetManager::new("test");
        mgr.add_item(make_item("sp1", "First")).unwrap();
        assert!(mgr.set_enabled("sp1", false).is_ok());
        assert!(!mgr.get_item("sp1").unwrap().enabled);
        assert_eq!(
            mgr.set_enabled("none", true),
            Err(OnlyofficeSpreadsheetError::NotFound("none".to_string()))
        );
    }
}
