/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CharaMakeClassEquip {
    pub row_id: u32,
    pub helmet: u64,
    pub top: u64,
    pub glove: u64,
    pub down: u64,
    pub shoes: u64,
    pub weapon: u64,
    pub sub_weapon: u64,
    pub class_id: u32,
    pub class: RowRef<ClassJob>,
}

impl Sheet for CharaMakeClassEquip {
    const SHEET_NAME: &'static str = "CharaMakeClassEquip";
}

impl FromExcelRow for CharaMakeClassEquip {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            helmet: single_row.columns.get(0).to_u64(),
            top: single_row.columns.get(1).to_u64(),
            glove: single_row.columns.get(2).to_u64(),
            down: single_row.columns.get(3).to_u64(),
            shoes: single_row.columns.get(4).to_u64(),
            weapon: single_row.columns.get(5).to_u64(),
            sub_weapon: single_row.columns.get(6).to_u64(),
            class_id: single_row.columns.get(7).to_u32(),
            class: RowRef::<ClassJob>::from(single_row.columns.get(7).to_u32()),
        })
    }
}

