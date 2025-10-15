/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AirshipExplorationLevel {
    pub row_id: u32,
    pub capacity: u16,
    pub exp_to_next: u32,
}

impl Sheet for AirshipExplorationLevel {
    const SHEET_NAME: &'static str = "AirshipExplorationLevel";
}

impl FromExcelRow for AirshipExplorationLevel {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            capacity: single_row.columns.get(0).to_u16(),
            exp_to_next: single_row.columns.get(1).to_u32(),
        })
    }
}

