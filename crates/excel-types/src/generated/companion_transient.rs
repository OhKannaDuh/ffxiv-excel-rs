/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CompanionTransient {
    pub row_id: u32,
    pub description: String,
    pub description_enhanced: String,
    pub tooltip: String,
    pub special_action_name: String,
    pub special_action_description: String,
    pub attack: u8,
    pub defense: u8,
    pub speed: u8,
    pub has_area_attack: bool,
    pub strength_gate: bool,
    pub strength_eye: bool,
    pub strength_shield: bool,
    pub strength_arcana: bool,
    pub minion_skill_type_id: u32,
    pub minion_skill_type: RowRef<MinionSkillType>,
}

impl Sheet for CompanionTransient {
    const SHEET_NAME: &'static str = "CompanionTransient";
}

impl FromExcelRow for CompanionTransient {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            description: single_row.columns.get(0).to_owned_string(),
            description_enhanced: single_row.columns.get(1).to_owned_string(),
            tooltip: single_row.columns.get(2).to_owned_string(),
            special_action_name: single_row.columns.get(3).to_owned_string(),
            special_action_description: single_row.columns.get(4).to_owned_string(),
            attack: single_row.columns.get(5).to_u8(),
            defense: single_row.columns.get(6).to_u8(),
            speed: single_row.columns.get(7).to_u8(),
            has_area_attack: single_row.columns.get(8).to_bit(0),
            strength_gate: single_row.columns.get(9).to_bit(1),
            strength_eye: single_row.columns.get(10).to_bit(2),
            strength_shield: single_row.columns.get(11).to_bit(3),
            strength_arcana: single_row.columns.get(12).to_bit(4),
            minion_skill_type_id: single_row.columns.get(13).to_u32(),
            minion_skill_type: RowRef::<MinionSkillType>::from(single_row.columns.get(13).to_u32()),
        })
    }
}

