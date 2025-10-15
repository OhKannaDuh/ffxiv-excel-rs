/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Relic3 {
    pub row_id: u32,
    pub item_animus_id: u32,
    pub item_animus: RowRef<Item>,
    pub item_scroll_id: u32,
    pub item_scroll: RowRef<Item>,
    pub materia_limit: u8,
    pub item_novus_id: u32,
    pub item_novus: RowRef<Item>,
    pub icon_id: u32,
}

impl Sheet for Relic3 {
    const SHEET_NAME: &'static str = "Relic3";
}

impl FromExcelRow for Relic3 {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_animus_id: single_row.columns.get(0).to_u32(),
            item_animus: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            item_scroll_id: single_row.columns.get(1).to_u32(),
            item_scroll: RowRef::<Item>::from(single_row.columns.get(1).to_u32()),
            materia_limit: single_row.columns.get(2).to_u8(),
            item_novus_id: single_row.columns.get(3).to_u32(),
            item_novus: RowRef::<Item>::from(single_row.columns.get(3).to_u32()),
            icon_id: single_row.columns.get(4).to_u32(),
        })
    }
}

