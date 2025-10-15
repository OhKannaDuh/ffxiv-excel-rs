/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MYCTemporaryItem {
    pub row_id: u32,
    pub category_id: u32,
    pub category: RowRef<MYCTemporaryItemUICategory>,
    pub _type: u8,
    pub action_id: u32,
    pub action: RowRef<Action>,
    pub max: u8,
    pub weight: u8,
    pub order: u8,
}

impl Sheet for MYCTemporaryItem {
    const SHEET_NAME: &'static str = "MYCTemporaryItem";
}

impl FromExcelRow for MYCTemporaryItem {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            category_id: single_row.columns.get(0).to_u32(),
            category: RowRef::<MYCTemporaryItemUICategory>::from(single_row.columns.get(0).to_u32()),
            _type: single_row.columns.get(1).to_u8(),
            action_id: single_row.columns.get(2).to_u32(),
            action: RowRef::<Action>::from(single_row.columns.get(2).to_u32()),
            max: single_row.columns.get(3).to_u8(),
            weight: single_row.columns.get(4).to_u8(),
            order: single_row.columns.get(5).to_u8(),
        })
    }
}

