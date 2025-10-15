/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJIRank {
    pub row_id: u32,
    pub exp_to_next: u32,
}

impl Sheet for MJIRank {
    const SHEET_NAME: &'static str = "MJIRank";
}

impl FromExcelRow for MJIRank {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            exp_to_next: single_row.columns.get(0).to_u32(),
        })
    }
}

