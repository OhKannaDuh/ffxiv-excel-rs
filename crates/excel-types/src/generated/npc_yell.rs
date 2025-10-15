/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct NpcYell {
    pub row_id: u32,
    pub output_type: u8,
    pub balloon_time: u32,
    pub is_balloon_slow: f32,
    pub battle_talk_time: f32,
    pub text: u8,
}

impl Sheet for NpcYell {
    const SHEET_NAME: &'static str = "NpcYell";
}

impl FromExcelRow for NpcYell {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            output_type: single_row.columns.get(4).to_u8(),
            balloon_time: single_row.columns.get(6).to_u32(),
            is_balloon_slow: single_row.columns.get(7).to_f32(),
            battle_talk_time: single_row.columns.get(8).to_f32(),
            text: single_row.columns.get(11).to_u8(),
        })
    }
}

