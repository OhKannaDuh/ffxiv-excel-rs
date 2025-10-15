/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CraftLeve {
    pub row_id: u32,
    pub leve_id: u32,
    pub leve: RowRef<Leve>,
    pub craft_leve_talk_id: u32,
    pub craft_leve_talk: RowRef<CraftLeveTalk>,
    pub repeats: u8,
}

impl Sheet for CraftLeve {
    const SHEET_NAME: &'static str = "CraftLeve";
}

impl FromExcelRow for CraftLeve {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            leve_id: single_row.columns.get(0).to_u32(),
            leve: RowRef::<Leve>::from(single_row.columns.get(0).to_u32()),
            craft_leve_talk_id: single_row.columns.get(1).to_u32(),
            craft_leve_talk: RowRef::<CraftLeveTalk>::from(single_row.columns.get(1).to_u32()),
            repeats: single_row.columns.get(2).to_u8(),
        })
    }
}

