/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJIRecipe {
    pub row_id: u32,
    pub log_message_id: u32,
    pub log_message: RowRef<LogMessage>,
    pub key_item_id: u32,
    pub key_item: RowRef<MJIKeyItem>,
    pub item_pouch_id: u32,
    pub item_pouch: RowRef<MJIItemPouch>,
    pub order: u8,
}

impl Sheet for MJIRecipe {
    const SHEET_NAME: &'static str = "MJIRecipe";
}

impl FromExcelRow for MJIRecipe {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            log_message_id: single_row.columns.get(0).to_u32(),
            log_message: RowRef::<LogMessage>::from(single_row.columns.get(0).to_u32()),
            key_item_id: single_row.columns.get(1).to_u32(),
            key_item: RowRef::<MJIKeyItem>::from(single_row.columns.get(1).to_u32()),
            item_pouch_id: single_row.columns.get(2).to_u32(),
            item_pouch: RowRef::<MJIItemPouch>::from(single_row.columns.get(2).to_u32()),
            order: single_row.columns.get(14).to_u8(),
        })
    }
}

