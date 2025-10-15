/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TraitTransient {
    pub row_id: u32,
    pub description: String,
}

impl Sheet for TraitTransient {
    const SHEET_NAME: &'static str = "TraitTransient";
}

impl FromExcelRow for TraitTransient {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            description: single_row.columns.get(0).to_owned_string(),
        })
    }
}

