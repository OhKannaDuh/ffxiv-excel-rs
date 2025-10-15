/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ContentExAction {
    pub row_id: u32,
    pub name_id: u32,
    pub name: RowRef<Action>,
    pub charges: u8,
}

impl Sheet for ContentExAction {
    const SHEET_NAME: &'static str = "ContentExAction";
}

impl FromExcelRow for ContentExAction {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name_id: single_row.columns.get(0).to_u32(),
            name: RowRef::<Action>::from(single_row.columns.get(0).to_u32()),
            charges: single_row.columns.get(2).to_u8(),
        })
    }
}

