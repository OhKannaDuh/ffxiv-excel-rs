/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct VaseFlower {
    pub row_id: u32,
    pub item_id: u32,
    pub item: RowRef<Item>,
}

impl Sheet for VaseFlower {
    const SHEET_NAME: &'static str = "VaseFlower";
}

impl FromExcelRow for VaseFlower {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_id: single_row.columns.get(3).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(3).to_u32()),
        })
    }
}

