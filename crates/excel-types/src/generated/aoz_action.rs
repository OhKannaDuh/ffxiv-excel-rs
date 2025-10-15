/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AozAction {
    pub row_id: u32,
    pub action_id: u32,
    pub action: RowRef<Action>,
    pub rank: u8,
}

impl Sheet for AozAction {
    const SHEET_NAME: &'static str = "AozAction";
}

impl FromExcelRow for AozAction {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            action_id: single_row.columns.get(0).to_u32(),
            action: RowRef::<Action>::from(single_row.columns.get(0).to_u32()),
            rank: single_row.columns.get(1).to_u8(),
        })
    }
}

