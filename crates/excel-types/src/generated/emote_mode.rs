/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct EmoteMode {
    pub row_id: u32,
    pub start_emote_id: u32,
    pub start_emote: RowRef<Emote>,
    pub end_emote_id: u32,
    pub end_emote: RowRef<Emote>,
    pub _move: bool,
    pub camera: bool,
    pub end_on_rotate: bool,
    pub end_on_emote: bool,
    pub condition_mode: u8,
}

impl Sheet for EmoteMode {
    const SHEET_NAME: &'static str = "EmoteMode";
}

impl FromExcelRow for EmoteMode {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            start_emote_id: single_row.columns.get(0).to_u32(),
            start_emote: RowRef::<Emote>::from(single_row.columns.get(0).to_u32()),
            end_emote_id: single_row.columns.get(1).to_u32(),
            end_emote: RowRef::<Emote>::from(single_row.columns.get(1).to_u32()),
            _move: single_row.columns.get(2).to_bit(0),
            camera: single_row.columns.get(3).to_bit(1),
            end_on_rotate: single_row.columns.get(4).to_bit(2),
            end_on_emote: single_row.columns.get(5).to_bit(3),
            condition_mode: single_row.columns.get(6).to_u8(),
        })
    }
}

