/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MotionTimeline {
    pub row_id: u32,
    pub filename: String,
    pub blend_group: u8,
    pub is_loop: bool,
    pub is_blink_enable: bool,
    pub is_lip_enable: bool,
}

impl Sheet for MotionTimeline {
    const SHEET_NAME: &'static str = "MotionTimeline";
}

impl FromExcelRow for MotionTimeline {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            filename: single_row.columns.get(0).to_owned_string(),
            blend_group: single_row.columns.get(1).to_u8(),
            is_loop: single_row.columns.get(2).to_bit(0),
            is_blink_enable: single_row.columns.get(3).to_bit(1),
            is_lip_enable: single_row.columns.get(4).to_bit(2),
        })
    }
}

