/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct VFX {
    pub row_id: u32,
    pub location: String,
}

impl Sheet for VFX {
    const SHEET_NAME: &'static str = "VFX";
}

impl FromExcelRow for VFX {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            location: single_row.columns.get(0).to_owned_string(),
        })
    }
}

