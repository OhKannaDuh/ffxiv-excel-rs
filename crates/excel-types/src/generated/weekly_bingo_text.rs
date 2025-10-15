/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct WeeklyBingoText {
    pub row_id: u32,
    pub description: String,
}

impl Sheet for WeeklyBingoText {
    const SHEET_NAME: &'static str = "WeeklyBingoText";
}

impl FromExcelRow for WeeklyBingoText {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            description: single_row.columns.get(0).to_owned_string(),
        })
    }
}

