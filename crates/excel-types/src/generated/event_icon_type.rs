/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct EventIconType {
    pub row_id: u32,
    pub npc_icon_available_id: u32,
    pub map_icon_available_id: u32,
    pub npc_icon_invalid_id: u32,
    pub map_icon_invalid_id: u32,
    pub icon_range: u8,
}

impl Sheet for EventIconType {
    const SHEET_NAME: &'static str = "EventIconType";
}

impl FromExcelRow for EventIconType {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            npc_icon_available_id: single_row.columns.get(0).to_u32(),
            map_icon_available_id: single_row.columns.get(1).to_u32(),
            npc_icon_invalid_id: single_row.columns.get(2).to_u32(),
            map_icon_invalid_id: single_row.columns.get(3).to_u32(),
            icon_range: single_row.columns.get(4).to_u8(),
        })
    }
}

