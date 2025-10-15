/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CharaMakeCustomize {
    pub row_id: u32,
    pub feature_id: u8,
    pub icon_id: u32,
    pub data: u16,
    pub is_purchasable: bool,
    pub hint_id: u32,
    pub hint: RowRef<Lobby>,
    pub hint_item_id: u32,
    pub hint_item: RowRef<Item>,
}

impl Sheet for CharaMakeCustomize {
    const SHEET_NAME: &'static str = "CharaMakeCustomize";
}

impl FromExcelRow for CharaMakeCustomize {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            feature_id: single_row.columns.get(0).to_u8(),
            icon_id: single_row.columns.get(1).to_u32(),
            data: single_row.columns.get(2).to_u16(),
            is_purchasable: single_row.columns.get(3).to_bit(0),
            hint_id: single_row.columns.get(4).to_u32(),
            hint: RowRef::<Lobby>::from(single_row.columns.get(4).to_u32()),
            hint_item_id: single_row.columns.get(5).to_u32(),
            hint_item: RowRef::<Item>::from(single_row.columns.get(5).to_u32()),
        })
    }
}

