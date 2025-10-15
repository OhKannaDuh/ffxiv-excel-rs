/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TreasureModel {
    pub row_id: u32,
    pub path: String,
}

impl Sheet for TreasureModel {
    const SHEET_NAME: &'static str = "TreasureModel";
}

impl FromExcelRow for TreasureModel {
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

