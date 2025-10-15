/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJIItemPouch {
    pub row_id: u32,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub category_id: u32,
    pub category: RowRef<MJIItemCategory>,
    pub crop_id: u32,
    pub crop: RowRef<MJICropSeed>,
    pub sort: u8,
}

impl Sheet for MJIItemPouch {
    const SHEET_NAME: &'static str = "MJIItemPouch";
}

impl FromExcelRow for MJIItemPouch {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_id: single_row.columns.get(0).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            category_id: single_row.columns.get(1).to_u32(),
            category: RowRef::<MJIItemCategory>::from(single_row.columns.get(1).to_u32()),
            crop_id: single_row.columns.get(2).to_u32(),
            crop: RowRef::<MJICropSeed>::from(single_row.columns.get(2).to_u32()),
            sort: single_row.columns.get(3).to_u8(),
        })
    }
}

