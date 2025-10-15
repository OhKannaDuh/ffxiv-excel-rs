/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Channeling {
    pub row_id: u32,
    pub file: String,
    pub width_scale: u8,
}

impl Sheet for Channeling {
    const SHEET_NAME: &'static str = "Channeling";
}

impl FromExcelRow for Channeling {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            file: single_row.columns.get(0).to_owned_string(),
            width_scale: single_row.columns.get(1).to_u8(),
        })
    }
}

