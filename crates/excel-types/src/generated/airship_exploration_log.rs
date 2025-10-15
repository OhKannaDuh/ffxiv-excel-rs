/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AirshipExplorationLog {
    pub row_id: u32,
    pub text: String,
}

impl Sheet for AirshipExplorationLog {
    const SHEET_NAME: &'static str = "AirshipExplorationLog";
}

impl FromExcelRow for AirshipExplorationLog {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            text: single_row.columns.get(0).to_owned_string(),
        })
    }
}

