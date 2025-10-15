/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJIAnimals {
    pub row_id: u32,
    pub b_npc_base_id: u32,
    pub b_npc_base: RowRef<BNpcBase>,
    pub size: u8,
    pub rarity: u8,
    pub sort: u8,
    pub icon_id: u32,
}

impl Sheet for MJIAnimals {
    const SHEET_NAME: &'static str = "MJIAnimals";
}

impl FromExcelRow for MJIAnimals {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            b_npc_base_id: single_row.columns.get(0).to_u32(),
            b_npc_base: RowRef::<BNpcBase>::from(single_row.columns.get(0).to_u32()),
            size: single_row.columns.get(1).to_u8(),
            rarity: single_row.columns.get(2).to_u8(),
            sort: single_row.columns.get(3).to_u8(),
            icon_id: single_row.columns.get(6).to_u32(),
        })
    }
}

