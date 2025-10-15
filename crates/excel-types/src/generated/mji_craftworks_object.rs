/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJICraftworksObject {
    pub row_id: u32,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub level_req: u16,
    pub crafting_time: u16,
    pub value: u16,
}

impl Sheet for MJICraftworksObject {
    const SHEET_NAME: &'static str = "MJICraftworksObject";
}

impl FromExcelRow for MJICraftworksObject {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_id: single_row.columns.get(0).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            level_req: single_row.columns.get(12).to_u16(),
            crafting_time: single_row.columns.get(13).to_u16(),
            value: single_row.columns.get(14).to_u16(),
        })
    }
}

