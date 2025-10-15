/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct YKW {
    pub row_id: u32,
    pub item_id: u32,
    pub item: RowRef<Item>,
}

impl Sheet for YKW {
    const SHEET_NAME: &'static str = "YKW";
}

impl FromExcelRow for YKW {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_id: single_row.columns.get(1).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

