/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FurnitureCatalogCategory {
    pub row_id: u32,
    pub category: String,
}

impl Sheet for FurnitureCatalogCategory {
    const SHEET_NAME: &'static str = "FurnitureCatalogCategory";
}

impl FromExcelRow for FurnitureCatalogCategory {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            category: single_row.columns.get(0).to_owned_string(),
        })
    }
}

