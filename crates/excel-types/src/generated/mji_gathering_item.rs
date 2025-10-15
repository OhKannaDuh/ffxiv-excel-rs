/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJIGatheringItem {
    pub row_id: u32,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub sort: u8,
    pub tool_id: u32,
    pub tool: RowRef<MJIGatheringTool>,
    pub x: i16,
    pub y: i16,
    pub radius: u16,
    pub map: u8,
}

impl Sheet for MJIGatheringItem {
    const SHEET_NAME: &'static str = "MJIGatheringItem";
}

impl FromExcelRow for MJIGatheringItem {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_id: single_row.columns.get(0).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            sort: single_row.columns.get(1).to_u8(),
            tool_id: single_row.columns.get(2).to_u32(),
            tool: RowRef::<MJIGatheringTool>::from(single_row.columns.get(2).to_u32()),
            x: single_row.columns.get(3).to_i16(),
            y: single_row.columns.get(4).to_i16(),
            radius: single_row.columns.get(5).to_u16(),
            map: single_row.columns.get(6).to_u8(),
        })
    }
}

