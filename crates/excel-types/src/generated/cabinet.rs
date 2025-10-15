/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Cabinet {
    pub row_id: u32,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub order: u16,
    pub category_id: u32,
    pub category: RowRef<CabinetCategory>,
    pub sort_key: u8,
}

impl Sheet for Cabinet {
    const SHEET_NAME: &'static str = "Cabinet";
}

impl FromExcelRow for Cabinet {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_id: single_row.columns.get(0).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            order: single_row.columns.get(1).to_u16(),
            category_id: single_row.columns.get(2).to_u32(),
            category: RowRef::<CabinetCategory>::from(single_row.columns.get(2).to_u32()),
            sort_key: single_row.columns.get(3).to_u8(),
        })
    }
}

