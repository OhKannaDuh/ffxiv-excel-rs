/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GFateRideShooting {
    pub row_id: u32,
    pub content_entry_id: u32,
}

impl Sheet for GFateRideShooting {
    const SHEET_NAME: &'static str = "GFateRideShooting";
}

impl FromExcelRow for GFateRideShooting {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            content_entry_id: single_row.columns.get(0).to_u32(),
        })
    }
}

