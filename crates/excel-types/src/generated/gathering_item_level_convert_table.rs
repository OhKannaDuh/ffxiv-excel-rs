/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GatheringItemLevelConvertTable {
    pub row_id: u32,
    pub gathering_item_level: u8,
    pub stars: u8,
}

impl Sheet for GatheringItemLevelConvertTable {
    const SHEET_NAME: &'static str = "GatheringItemLevelConvertTable";
}

impl FromExcelRow for GatheringItemLevelConvertTable {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            gathering_item_level: single_row.columns.get(0).to_u8(),
            stars: single_row.columns.get(1).to_u8(),
        })
    }
}

