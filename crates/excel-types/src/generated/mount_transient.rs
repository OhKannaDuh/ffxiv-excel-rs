/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MountTransient {
    pub row_id: u32,
    pub description: String,
    pub description_enhanced: String,
    pub tooltip: String,
}

impl Sheet for MountTransient {
    const SHEET_NAME: &'static str = "MountTransient";
}

impl FromExcelRow for MountTransient {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            description: single_row.columns.get(0).to_owned_string(),
            description_enhanced: single_row.columns.get(1).to_owned_string(),
            tooltip: single_row.columns.get(2).to_owned_string(),
        })
    }
}

