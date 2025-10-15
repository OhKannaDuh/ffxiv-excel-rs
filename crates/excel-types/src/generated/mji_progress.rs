/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJIProgress {
    pub row_id: u32,
    pub vision: String,
    pub objective: String,
    pub previous_objective: String,
}

impl Sheet for MJIProgress {
    const SHEET_NAME: &'static str = "MJIProgress";
}

impl FromExcelRow for MJIProgress {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            vision: single_row.columns.get(0).to_owned_string(),
            objective: single_row.columns.get(1).to_owned_string(),
            previous_objective: single_row.columns.get(2).to_owned_string(),
        })
    }
}

