/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct SatisfactionNpc {
    pub row_id: u32,
    pub npc_id: u32,
    pub npc: RowRef<ENpcResident>,
    pub quest_required_id: u32,
    pub quest_required: RowRef<Quest>,
    pub level_unlock: u8,
    pub deliveries_per_week: u8,
    pub icon_id: u32,
}

impl Sheet for SatisfactionNpc {
    const SHEET_NAME: &'static str = "SatisfactionNpc";
}

impl FromExcelRow for SatisfactionNpc {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            npc_id: single_row.columns.get(0).to_u32(),
            npc: RowRef::<ENpcResident>::from(single_row.columns.get(0).to_u32()),
            quest_required_id: single_row.columns.get(1).to_u32(),
            quest_required: RowRef::<Quest>::from(single_row.columns.get(1).to_u32()),
            level_unlock: single_row.columns.get(2).to_u8(),
            deliveries_per_week: single_row.columns.get(3).to_u8(),
            icon_id: single_row.columns.get(88).to_u32(),
        })
    }
}

