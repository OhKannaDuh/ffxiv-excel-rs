/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BGMFadeType {
    pub row_id: u32,
    pub fade_out_time: f32,
    pub fade_in_time: f32,
    pub fade_in_start_time: f32,
    pub resume_fade_in_time: f32,
}

impl Sheet for BGMFadeType {
    const SHEET_NAME: &'static str = "BGMFadeType";
}

impl FromExcelRow for BGMFadeType {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            fade_out_time: single_row.columns.get(0).to_f32(),
            fade_in_time: single_row.columns.get(1).to_f32(),
            fade_in_start_time: single_row.columns.get(2).to_f32(),
            resume_fade_in_time: single_row.columns.get(3).to_f32(),
        })
    }
}

