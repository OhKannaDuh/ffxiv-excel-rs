/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJIGardenscaping {
    pub row_id: u32,
    pub level: u8,
    pub item_id: u32,
    pub item: RowRef<Item>,
}

impl Sheet for MJIGardenscaping {
    const SHEET_NAME: &'static str = "MJIGardenscaping";
}

impl FromExcelRow for MJIGardenscaping {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            level: single_row.columns.get(0).to_u8(),
            item_id: single_row.columns.get(5).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(5).to_u32()),
        })
    }
}

