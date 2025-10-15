/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct SubmarineRank {
    pub row_id: u32,
    pub capacity: u16,
    pub exp_to_next: u32,
    pub surveillance_bonus: u8,
    pub retrieval_bonus: u8,
    pub speed_bonus: u8,
    pub range_bonus: u8,
    pub favor_bonus: u8,
}

impl Sheet for SubmarineRank {
    const SHEET_NAME: &'static str = "SubmarineRank";
}

impl FromExcelRow for SubmarineRank {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            capacity: single_row.columns.get(0).to_u16(),
            exp_to_next: single_row.columns.get(1).to_u32(),
            surveillance_bonus: single_row.columns.get(2).to_u8(),
            retrieval_bonus: single_row.columns.get(3).to_u8(),
            speed_bonus: single_row.columns.get(4).to_u8(),
            range_bonus: single_row.columns.get(5).to_u8(),
            favor_bonus: single_row.columns.get(6).to_u8(),
        })
    }
}

