/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Companion {
    pub row_id: u32,
    pub singular: String,
    pub adjective: i8,
    pub plural: String,
    pub possessive_pronoun: i8,
    pub starts_with_vowel: i8,
    pub pronoun: i8,
    pub article: i8,
    pub model_id: u32,
    pub model: RowRef<ModelChara>,
    pub scale: u8,
    pub inactive_idle_0: u8,
    pub inactive_idle_1: u8,
    pub inactive_battle: u8,
    pub inactive_wandering: u8,
    pub behavior_id: u32,
    pub behavior: RowRef<CompanionMove>,
    pub special: u8,
    pub wandering_wait: u8,
    pub priority: u8,
    pub enemy: bool,
    pub battle: bool,
    pub roulette: u16,
    pub icon_id: u32,
    pub order: u16,
    pub idle_animation: bool,
    pub cost: u8,
    pub hp: u16,
    pub skill_angle: u16,
    pub skill_cost: u8,
    pub minion_race_id: u32,
    pub minion_race: RowRef<MinionRace>,
}

impl Sheet for Companion {
    const SHEET_NAME: &'static str = "Companion";
}

impl FromExcelRow for Companion {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            singular: single_row.columns.get(0).to_owned_string(),
            adjective: single_row.columns.get(1).to_i8(),
            plural: single_row.columns.get(2).to_owned_string(),
            possessive_pronoun: single_row.columns.get(3).to_i8(),
            starts_with_vowel: single_row.columns.get(4).to_i8(),
            pronoun: single_row.columns.get(6).to_i8(),
            article: single_row.columns.get(7).to_i8(),
            model_id: single_row.columns.get(8).to_u32(),
            model: RowRef::<ModelChara>::from(single_row.columns.get(8).to_u32()),
            scale: single_row.columns.get(9).to_u8(),
            inactive_idle_0: single_row.columns.get(10).to_u8(),
            inactive_idle_1: single_row.columns.get(11).to_u8(),
            inactive_battle: single_row.columns.get(12).to_u8(),
            inactive_wandering: single_row.columns.get(13).to_u8(),
            behavior_id: single_row.columns.get(14).to_u32(),
            behavior: RowRef::<CompanionMove>::from(single_row.columns.get(14).to_u32()),
            special: single_row.columns.get(15).to_u8(),
            wandering_wait: single_row.columns.get(16).to_u8(),
            priority: single_row.columns.get(17).to_u8(),
            enemy: single_row.columns.get(23).to_bit(3),
            battle: single_row.columns.get(24).to_bit(4),
            roulette: single_row.columns.get(25).to_u16(),
            icon_id: single_row.columns.get(28).to_u32(),
            order: single_row.columns.get(29).to_u16(),
            idle_animation: single_row.columns.get(30).to_bit(7),
            cost: single_row.columns.get(32).to_u8(),
            hp: single_row.columns.get(33).to_u16(),
            skill_angle: single_row.columns.get(35).to_u16(),
            skill_cost: single_row.columns.get(36).to_u8(),
            minion_race_id: single_row.columns.get(39).to_u32(),
            minion_race: RowRef::<MinionRace>::from(single_row.columns.get(39).to_u32()),
        })
    }
}

