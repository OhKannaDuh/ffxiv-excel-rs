/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Buddy {
    pub row_id: u32,
    pub base: u8,
    pub quest_requirement_2_id: u32,
    pub quest_requirement_2: RowRef<Quest>,
    pub quest_requirement_1_id: u32,
    pub quest_requirement_1: RowRef<Quest>,
    pub base_equip: i32,
    pub sound_effect_4: String,
    pub sound_effect_3: String,
    pub sound_effect_2: String,
    pub sound_effect_1: String,
}

impl Sheet for Buddy {
    const SHEET_NAME: &'static str = "Buddy";
}

impl FromExcelRow for Buddy {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            base: single_row.columns.get(0).to_u8(),
            quest_requirement_2_id: single_row.columns.get(1).to_u32(),
            quest_requirement_2: RowRef::<Quest>::from(single_row.columns.get(1).to_u32()),
            quest_requirement_1_id: single_row.columns.get(2).to_u32(),
            quest_requirement_1: RowRef::<Quest>::from(single_row.columns.get(2).to_u32()),
            base_equip: single_row.columns.get(3).to_i32(),
            sound_effect_4: single_row.columns.get(4).to_owned_string(),
            sound_effect_3: single_row.columns.get(5).to_owned_string(),
            sound_effect_2: single_row.columns.get(6).to_owned_string(),
            sound_effect_1: single_row.columns.get(7).to_owned_string(),
        })
    }
}

