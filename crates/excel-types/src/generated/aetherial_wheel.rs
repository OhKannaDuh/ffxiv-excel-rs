/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AetherialWheel {
    pub row_id: u32,
    pub item_unprimed_id: u32,
    pub item_unprimed: RowRef<Item>,
    pub item_primed_id: u32,
    pub item_primed: RowRef<Item>,
    pub grade: u8,
    pub hours_required: u8,
}

impl Sheet for AetherialWheel {
    const SHEET_NAME: &'static str = "AetherialWheel";
}

impl FromExcelRow for AetherialWheel {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_unprimed_id: single_row.columns.get(0).to_u32(),
            item_unprimed: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            item_primed_id: single_row.columns.get(1).to_u32(),
            item_primed: RowRef::<Item>::from(single_row.columns.get(1).to_u32()),
            grade: single_row.columns.get(2).to_u8(),
            hours_required: single_row.columns.get(3).to_u8(),
        })
    }
}

