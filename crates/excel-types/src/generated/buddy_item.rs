/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BuddyItem {
    pub row_id: u32,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub use_field: bool,
    pub use_training: bool,
    pub status: u8,
}

impl Sheet for BuddyItem {
    const SHEET_NAME: &'static str = "BuddyItem";
}

impl FromExcelRow for BuddyItem {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_id: single_row.columns.get(0).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            use_field: single_row.columns.get(1).to_bit(0),
            use_training: single_row.columns.get(2).to_bit(1),
            status: single_row.columns.get(4).to_u8(),
        })
    }
}

