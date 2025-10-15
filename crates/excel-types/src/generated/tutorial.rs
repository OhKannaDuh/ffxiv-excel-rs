/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Tutorial {
    pub row_id: u32,
    pub exp: u8,
    pub gil: u32,
    pub reward_tank_id: u32,
    pub reward_tank: RowRef<Item>,
    pub reward_melee_id: u32,
    pub reward_melee: RowRef<Item>,
    pub reward_ranged_id: u32,
    pub reward_ranged: RowRef<Item>,
    pub objective_id: u32,
    pub objective: RowRef<InstanceContentTextData>,
}

impl Sheet for Tutorial {
    const SHEET_NAME: &'static str = "Tutorial";
}

impl FromExcelRow for Tutorial {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            exp: single_row.columns.get(4).to_u8(),
            gil: single_row.columns.get(5).to_u32(),
            reward_tank_id: single_row.columns.get(6).to_u32(),
            reward_tank: RowRef::<Item>::from(single_row.columns.get(6).to_u32()),
            reward_melee_id: single_row.columns.get(7).to_u32(),
            reward_melee: RowRef::<Item>::from(single_row.columns.get(7).to_u32()),
            reward_ranged_id: single_row.columns.get(8).to_u32(),
            reward_ranged: RowRef::<Item>::from(single_row.columns.get(8).to_u32()),
            objective_id: single_row.columns.get(9).to_u32(),
            objective: RowRef::<InstanceContentTextData>::from(single_row.columns.get(9).to_u32()),
        })
    }
}

