/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AOZReport {
    pub row_id: u32,
    pub reward_id: u32,
    pub order: i8,
}

impl Sheet for AOZReport {
    const SHEET_NAME: &'static str = "AOZReport";
}

impl FromExcelRow for AOZReport {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            reward_id: single_row.columns.get(1).to_u32(),
            order: single_row.columns.get(2).to_i8(),
        })
    }
}

