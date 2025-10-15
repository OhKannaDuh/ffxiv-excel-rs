/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct LeveString {
    pub row_id: u32,
    pub objective: String,
}

impl Sheet for LeveString {
    const SHEET_NAME: &'static str = "LeveString";
}

impl FromExcelRow for LeveString {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            objective: single_row.columns.get(0).to_owned_string(),
        })
    }
}

