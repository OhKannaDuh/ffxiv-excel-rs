/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MonsterNoteTarget {
    pub row_id: u32,
    pub b_npc_name_id: u32,
    pub b_npc_name: RowRef<BNpcName>,
    pub icon_id: u32,
    pub town_id: u32,
    pub town: RowRef<Town>,
}

impl Sheet for MonsterNoteTarget {
    const SHEET_NAME: &'static str = "MonsterNoteTarget";
}

impl FromExcelRow for MonsterNoteTarget {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            b_npc_name_id: single_row.columns.get(0).to_u32(),
            b_npc_name: RowRef::<BNpcName>::from(single_row.columns.get(0).to_u32()),
            icon_id: single_row.columns.get(1).to_u32(),
            town_id: single_row.columns.get(2).to_u32(),
            town: RowRef::<Town>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

