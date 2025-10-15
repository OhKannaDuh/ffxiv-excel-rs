/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AozActionTransient {
    pub row_id: u32,
    pub number: u8,
    pub icon_id: u32,
    pub stats: String,
    pub description: String,
    pub location_key: u8,
    pub location_id: u32,
    pub required_for_quest_id: u32,
    pub required_for_quest: RowRef<Quest>,
    pub previous_quest_id: u32,
    pub previous_quest: RowRef<Quest>,
    pub targets_enemy: bool,
    pub targets_self_or_ally: bool,
    pub cause_slow: bool,
    pub cause_petrify: bool,
    pub cause_paralysis: bool,
    pub cause_interrupt: bool,
    pub cause_blind: bool,
    pub cause_stun: bool,
    pub cause_sleep: bool,
    pub cause_bind: bool,
    pub cause_heavy: bool,
    pub cause_death: bool,
}

impl Sheet for AozActionTransient {
    const SHEET_NAME: &'static str = "AozActionTransient";
}

impl FromExcelRow for AozActionTransient {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            number: single_row.columns.get(0).to_u8(),
            icon_id: single_row.columns.get(1).to_u32(),
            stats: single_row.columns.get(2).to_owned_string(),
            description: single_row.columns.get(3).to_owned_string(),
            location_key: single_row.columns.get(4).to_u8(),
            location_id: single_row.columns.get(5).to_u32(),
            required_for_quest_id: single_row.columns.get(6).to_u32(),
            required_for_quest: RowRef::<Quest>::from(single_row.columns.get(6).to_u32()),
            previous_quest_id: single_row.columns.get(7).to_u32(),
            previous_quest: RowRef::<Quest>::from(single_row.columns.get(7).to_u32()),
            targets_enemy: single_row.columns.get(8).to_bit(0),
            targets_self_or_ally: single_row.columns.get(9).to_bit(1),
            cause_slow: single_row.columns.get(10).to_bit(2),
            cause_petrify: single_row.columns.get(11).to_bit(3),
            cause_paralysis: single_row.columns.get(12).to_bit(4),
            cause_interrupt: single_row.columns.get(13).to_bit(5),
            cause_blind: single_row.columns.get(14).to_bit(6),
            cause_stun: single_row.columns.get(15).to_bit(7),
            cause_sleep: single_row.columns.get(16).to_bit(0),
            cause_bind: single_row.columns.get(17).to_bit(1),
            cause_heavy: single_row.columns.get(18).to_bit(2),
            cause_death: single_row.columns.get(19).to_bit(3),
        })
    }
}

