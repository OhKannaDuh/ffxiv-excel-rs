/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GuildOrder {
    pub row_id: u32,
    pub e_npc_name_id: u32,
    pub e_npc_name: RowRef<ENpcResident>,
    pub objective: String,
    pub description_1: String,
    pub description_2: String,
    pub description_3: String,
    pub completion_bonus_exp: u32,
    pub reward_exp: u32,
    pub completion_bonus_gil: u32,
    pub reward_gil: u32,
}

impl Sheet for GuildOrder {
    const SHEET_NAME: &'static str = "GuildOrder";
}

impl FromExcelRow for GuildOrder {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            e_npc_name_id: single_row.columns.get(0).to_u32(),
            e_npc_name: RowRef::<ENpcResident>::from(single_row.columns.get(0).to_u32()),
            objective: single_row.columns.get(1).to_owned_string(),
            description_1: single_row.columns.get(2).to_owned_string(),
            description_2: single_row.columns.get(3).to_owned_string(),
            description_3: single_row.columns.get(4).to_owned_string(),
            completion_bonus_exp: single_row.columns.get(5).to_u32(),
            reward_exp: single_row.columns.get(6).to_u32(),
            completion_bonus_gil: single_row.columns.get(7).to_u32(),
            reward_gil: single_row.columns.get(8).to_u32(),
        })
    }
}

