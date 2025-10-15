/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Title {
    pub row_id: u32,
    pub masculine: String,
    pub feminine: String,
    pub is_prefix: bool,
    pub order: u16,
}

impl Sheet for Title {
    const SHEET_NAME: &'static str = "Title";
}

impl FromExcelRow for Title {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            masculine: single_row.columns.get(0).to_owned_string(),
            feminine: single_row.columns.get(1).to_owned_string(),
            is_prefix: single_row.columns.get(2).to_bit(0),
            order: single_row.columns.get(3).to_u16(),
        })
    }
}

