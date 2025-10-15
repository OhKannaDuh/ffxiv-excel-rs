/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct QuestDefineClient {
    pub row_id: u32,
    pub target_id: u32,
    pub target_item: RowRef<Item>,
    pub target_quest: RowRef<Quest>,
    pub target_e_npc_base: RowRef<ENpcBase>,
    pub target_e_obj_name: RowRef<EObjName>,
    pub target_level: RowRef<Level>,
}

impl Sheet for QuestDefineClient {
    const SHEET_NAME: &'static str = "QuestDefineClient";
}

impl FromExcelRow for QuestDefineClient {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            target_id: single_row.columns.get(1).to_u32(),
            target_item: RowRef::<Item>::from(single_row.columns.get(1).to_u32()),
            target_quest: RowRef::<Quest>::from(single_row.columns.get(1).to_u32()),
            target_e_npc_base: RowRef::<ENpcBase>::from(single_row.columns.get(1).to_u32()),
            target_e_obj_name: RowRef::<EObjName>::from(single_row.columns.get(1).to_u32()),
            target_level: RowRef::<Level>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

