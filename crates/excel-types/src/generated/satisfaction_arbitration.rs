/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct SatisfactionArbitration {
    pub row_id: u32,
    pub satisfaction_level: u8,
    pub satisfaction_npc_id: u32,
    pub satisfaction_npc: RowRef<SatisfactionNpc>,
    pub quest_id: u32,
    pub quest: RowRef<Quest>,
}

impl Sheet for SatisfactionArbitration {
    const SHEET_NAME: &'static str = "SatisfactionArbitration";
}

impl FromExcelRow for SatisfactionArbitration {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            satisfaction_level: single_row.columns.get(0).to_u8(),
            satisfaction_npc_id: single_row.columns.get(1).to_u32(),
            satisfaction_npc: RowRef::<SatisfactionNpc>::from(single_row.columns.get(1).to_u32()),
            quest_id: single_row.columns.get(2).to_u32(),
            quest: RowRef::<Quest>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

