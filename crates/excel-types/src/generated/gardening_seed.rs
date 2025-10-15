/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GardeningSeed {
    pub row_id: u32,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub model_id: u16,
    pub icon_id: u32,
    pub se: bool,
}

impl Sheet for GardeningSeed {
    const SHEET_NAME: &'static str = "GardeningSeed";
}

impl FromExcelRow for GardeningSeed {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_id: single_row.columns.get(0).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            model_id: single_row.columns.get(1).to_u16(),
            icon_id: single_row.columns.get(2).to_u32(),
            se: single_row.columns.get(3).to_bit(0),
        })
    }
}

