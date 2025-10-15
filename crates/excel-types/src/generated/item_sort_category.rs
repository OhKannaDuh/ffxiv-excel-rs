/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ItemSortCategory {
    pub row_id: u32,
    pub param: u8,
}

impl Sheet for ItemSortCategory {
    const SHEET_NAME: &'static str = "ItemSortCategory";
}

impl FromExcelRow for ItemSortCategory {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            param: single_row.columns.get(0).to_u8(),
        })
    }
}

