/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Carry {
    pub row_id: u32,
    pub model: u64,
    pub timeline: u8,
}

impl Sheet for Carry {
    const SHEET_NAME: &'static str = "Carry";
}

impl FromExcelRow for Carry {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            model: single_row.columns.get(0).to_u64(),
            timeline: single_row.columns.get(1).to_u8(),
        })
    }
}

