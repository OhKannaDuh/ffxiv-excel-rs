/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Balloon {
    pub row_id: u32,
    pub slowly: bool,
    pub dialogue: String,
}

impl Sheet for Balloon {
    const SHEET_NAME: &'static str = "Balloon";
}

impl FromExcelRow for Balloon {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            slowly: single_row.columns.get(0).to_bit(0),
            dialogue: single_row.columns.get(1).to_owned_string(),
        })
    }
}

