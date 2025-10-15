/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Resident {
    pub row_id: u32,
    pub model: u64,
    pub npc_yell_id: u32,
    pub npc_yell: RowRef<NpcYell>,
    pub resident_motion_type_id: u32,
}

impl Sheet for Resident {
    const SHEET_NAME: &'static str = "Resident";
}

impl FromExcelRow for Resident {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            model: single_row.columns.get(1).to_u64(),
            npc_yell_id: single_row.columns.get(2).to_u32(),
            npc_yell: RowRef::<NpcYell>::from(single_row.columns.get(2).to_u32()),
            resident_motion_type_id: single_row.columns.get(4).to_u32(),
        })
    }
}

