/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct StainTransient {
    pub row_id: u32,
    pub item_1_id: u32,
    pub item_1: RowRef<Item>,
    pub item_2_id: u32,
    pub item_2: RowRef<Item>,
}

impl Sheet for StainTransient {
    const SHEET_NAME: &'static str = "StainTransient";
}

impl FromExcelRow for StainTransient {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_1_id: single_row.columns.get(0).to_u32(),
            item_1: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            item_2_id: single_row.columns.get(1).to_u32(),
            item_2: RowRef::<Item>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

