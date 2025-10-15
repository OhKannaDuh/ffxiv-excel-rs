/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BuddyRank {
    pub row_id: u32,
    pub exp_required: u32,
}

impl Sheet for BuddyRank {
    const SHEET_NAME: &'static str = "BuddyRank";
}

impl FromExcelRow for BuddyRank {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            exp_required: single_row.columns.get(0).to_u32(),
        })
    }
}

