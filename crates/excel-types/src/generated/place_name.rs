/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct PlaceName {
    pub row_id: u32,
    pub name: String,
    pub name_no_article: String,
}

impl Sheet for PlaceName {
    const SHEET_NAME: &'static str = "PlaceName";
}

impl FromExcelRow for PlaceName {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            name_no_article: single_row.columns.get(2).to_owned_string(),
        })
    }
}

