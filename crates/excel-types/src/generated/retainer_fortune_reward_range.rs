/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RetainerFortuneRewardRange {
    pub row_id: u32,
    pub percent_of_level: u16,
}

impl Sheet for RetainerFortuneRewardRange {
    const SHEET_NAME: &'static str = "RetainerFortuneRewardRange";
}

impl FromExcelRow for RetainerFortuneRewardRange {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            percent_of_level: single_row.columns.get(0).to_u16(),
        })
    }
}

