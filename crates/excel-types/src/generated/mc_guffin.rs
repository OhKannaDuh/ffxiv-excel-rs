/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct McGuffin {
    pub row_id: u32,
    pub ui_data_id: u32,
    pub ui_data: RowRef<McGuffinUIData>,
}

impl Sheet for McGuffin {
    const SHEET_NAME: &'static str = "McGuffin";
}

impl FromExcelRow for McGuffin {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            ui_data_id: single_row.columns.get(0).to_u32(),
            ui_data: RowRef::<McGuffinUIData>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

