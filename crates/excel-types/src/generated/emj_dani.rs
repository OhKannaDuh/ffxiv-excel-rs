/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct EmjDani {
    pub row_id: u32,
    pub icon_id: u32,
}

impl Sheet for EmjDani {
    const SHEET_NAME: &'static str = "EmjDani";
}

impl FromExcelRow for EmjDani {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            icon_id: single_row.columns.get(0).to_u32(),
        })
    }
}

