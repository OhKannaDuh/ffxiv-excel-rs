/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BehaviorPath {
    pub row_id: u32,
    pub is_turn_transition: bool,
    pub is_fade_out: bool,
    pub is_fade_in: bool,
    pub is_walking: bool,
    pub speed: f32,
}

impl Sheet for BehaviorPath {
    const SHEET_NAME: &'static str = "BehaviorPath";
}

impl FromExcelRow for BehaviorPath {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            is_turn_transition: single_row.columns.get(0).to_bit(0),
            is_fade_out: single_row.columns.get(1).to_bit(1),
            is_fade_in: single_row.columns.get(2).to_bit(2),
            is_walking: single_row.columns.get(3).to_bit(3),
            speed: single_row.columns.get(5).to_f32(),
        })
    }
}

