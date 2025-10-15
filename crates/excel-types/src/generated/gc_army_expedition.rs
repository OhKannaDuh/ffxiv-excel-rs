/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GcArmyExpedition {
    pub row_id: u32,
    pub required_flag: u8,
    pub unlock_flag: u8,
    pub required_level: u8,
    pub required_seals: u16,
    pub reward_experience: u32,
    pub percent_base: u8,
    pub gc_army_expedition_type_id: u32,
    pub gc_army_expedition_type: RowRef<GcArmyExpeditionType>,
    pub name: String,
    pub description: String,
}

impl Sheet for GcArmyExpedition {
    const SHEET_NAME: &'static str = "GcArmyExpedition";
}

impl FromExcelRow for GcArmyExpedition {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            required_flag: single_row.columns.get(0).to_u8(),
            unlock_flag: single_row.columns.get(1).to_u8(),
            required_level: single_row.columns.get(2).to_u8(),
            required_seals: single_row.columns.get(3).to_u16(),
            reward_experience: single_row.columns.get(4).to_u32(),
            percent_base: single_row.columns.get(5).to_u8(),
            gc_army_expedition_type_id: single_row.columns.get(7).to_u32(),
            gc_army_expedition_type: RowRef::<GcArmyExpeditionType>::from(single_row.columns.get(7).to_u32()),
            name: single_row.columns.get(8).to_owned_string(),
            description: single_row.columns.get(9).to_owned_string(),
        })
    }
}

