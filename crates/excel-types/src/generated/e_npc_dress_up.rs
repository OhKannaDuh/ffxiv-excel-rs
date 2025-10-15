/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ENpcDressUp {
    pub row_id: u32,
    pub e_npc_dress_up_dress_id: u32,
    pub e_npc_dress_up_dress: RowRef<ENpcDressUpDress>,
}

impl Sheet for ENpcDressUp {
    const SHEET_NAME: &'static str = "ENpcDressUp";
}

impl FromExcelRow for ENpcDressUp {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            e_npc_dress_up_dress_id: single_row.columns.get(1).to_u32(),
            e_npc_dress_up_dress: RowRef::<ENpcDressUpDress>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

