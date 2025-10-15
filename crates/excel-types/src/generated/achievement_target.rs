/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AchievementTarget {
    pub row_id: u32,
    pub _type: u8,
    pub value: u32,
}

impl Sheet for AchievementTarget {
    const SHEET_NAME: &'static str = "AchievementTarget";
}

impl FromExcelRow for AchievementTarget {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _type: single_row.columns.get(0).to_u8(),
            value: single_row.columns.get(1).to_u32(),
        })
    }
}

