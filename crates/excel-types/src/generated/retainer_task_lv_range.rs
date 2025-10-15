/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RetainerTaskLvRange {
    pub row_id: u32,
    pub min: u8,
    pub max: u8,
}

impl Sheet for RetainerTaskLvRange {
    const SHEET_NAME: &'static str = "RetainerTaskLvRange";
}

impl FromExcelRow for RetainerTaskLvRange {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            min: single_row.columns.get(0).to_u8(),
            max: single_row.columns.get(1).to_u8(),
        })
    }
}

