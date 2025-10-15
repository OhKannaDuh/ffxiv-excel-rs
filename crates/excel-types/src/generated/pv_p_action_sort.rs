/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct PvPActionSort {
    pub row_id: u32,
    pub action_type: u8,
    pub action_id: u32,
}

impl Sheet for PvPActionSort {
    const SHEET_NAME: &'static str = "PvPActionSort";
}

impl FromExcelRow for PvPActionSort {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            action_type: single_row.columns.get(0).to_u8(),
            action_id: single_row.columns.get(1).to_u32(),
        })
    }
}

