/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BGM {
    pub row_id: u32,
    pub file: String,
    pub priority: u8,
    pub disable_restart_time_out: bool,
    pub disable_restart: bool,
    pub pass_end: bool,
    pub disable_restart_reset_time: f32,
    pub special_mode: u8,
}

impl Sheet for BGM {
    const SHEET_NAME: &'static str = "BGM";
}

impl FromExcelRow for BGM {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            file: single_row.columns.get(0).to_owned_string(),
            priority: single_row.columns.get(1).to_u8(),
            disable_restart_time_out: single_row.columns.get(2).to_bit(0),
            disable_restart: single_row.columns.get(3).to_bit(1),
            pass_end: single_row.columns.get(4).to_bit(2),
            disable_restart_reset_time: single_row.columns.get(5).to_f32(),
            special_mode: single_row.columns.get(6).to_u8(),
        })
    }
}

