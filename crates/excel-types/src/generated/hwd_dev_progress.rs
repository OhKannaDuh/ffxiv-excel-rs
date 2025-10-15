/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HWDDevProgress {
    pub row_id: u32,
    pub can_go_next: bool,
}

impl Sheet for HWDDevProgress {
    const SHEET_NAME: &'static str = "HWDDevProgress";
}

impl FromExcelRow for HWDDevProgress {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            can_go_next: single_row.columns.get(0).to_bit(0),
        })
    }
}

