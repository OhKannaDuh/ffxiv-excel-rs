/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ParamGrow {
    pub row_id: u32,
    pub exp_to_next: i32,
    pub additional_actions: u8,
    pub apply_action: u8,
    pub scaled_quest_xp: u16,
    pub mp_modifier: i32,
    pub base_speed: i32,
    pub level_modifier: i32,
    pub quest_exp_modifier: u8,
    pub hp_modifier: u16,
    pub hunting_log_exp_reward: i32,
    pub monster_note_seals: i32,
    pub item_level_sync: u16,
    pub proper_dungeon: u16,
    pub proper_guild_order: u16,
    pub crafting_level: u16,
}

impl Sheet for ParamGrow {
    const SHEET_NAME: &'static str = "ParamGrow";
}

impl FromExcelRow for ParamGrow {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            exp_to_next: single_row.columns.get(0).to_i32(),
            additional_actions: single_row.columns.get(1).to_u8(),
            apply_action: single_row.columns.get(2).to_u8(),
            scaled_quest_xp: single_row.columns.get(3).to_u16(),
            mp_modifier: single_row.columns.get(4).to_i32(),
            base_speed: single_row.columns.get(5).to_i32(),
            level_modifier: single_row.columns.get(6).to_i32(),
            quest_exp_modifier: single_row.columns.get(7).to_u8(),
            hp_modifier: single_row.columns.get(8).to_u16(),
            hunting_log_exp_reward: single_row.columns.get(9).to_i32(),
            monster_note_seals: single_row.columns.get(10).to_i32(),
            item_level_sync: single_row.columns.get(11).to_u16(),
            proper_dungeon: single_row.columns.get(12).to_u16(),
            proper_guild_order: single_row.columns.get(13).to_u16(),
            crafting_level: single_row.columns.get(14).to_u16(),
        })
    }
}

