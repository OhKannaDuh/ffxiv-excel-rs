/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJIRecipeMaterial {
    pub row_id: u32,
    pub item_pouch_id: u32,
    pub item_pouch: RowRef<MJIItemPouch>,
}

impl Sheet for MJIRecipeMaterial {
    const SHEET_NAME: &'static str = "MJIRecipeMaterial";
}

impl FromExcelRow for MJIRecipeMaterial {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_pouch_id: single_row.columns.get(0).to_u32(),
            item_pouch: RowRef::<MJIItemPouch>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

