/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJIItemCategory {
    pub row_id: u32,
    pub singular: String,
    pub plural: String,
}

impl Sheet for MJIItemCategory {
    const SHEET_NAME: &'static str = "MJIItemCategory";
}

impl FromExcelRow for MJIItemCategory {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            singular: single_row.columns.get(0).to_owned_string(),
            plural: single_row.columns.get(1).to_owned_string(),
        })
    }
}

