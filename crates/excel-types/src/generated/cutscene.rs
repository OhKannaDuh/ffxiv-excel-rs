/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Cutscene {
    pub row_id: u32,
    pub path: String,
}

impl Sheet for Cutscene {
    const SHEET_NAME: &'static str = "Cutscene";
}

impl FromExcelRow for Cutscene {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            path: single_row.columns.get(0).to_owned_string(),
        })
    }
}

