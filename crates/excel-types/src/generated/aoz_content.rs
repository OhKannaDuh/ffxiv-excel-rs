/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AOZContent {
    pub row_id: u32,
    pub standard_finish_time: u16,
    pub ideal_finish_time: u16,
    pub act1_fight_type: u8,
    pub act1_id: u32,
    pub arena_type_1: u8,
    pub act2_fight_type: u8,
    pub act2_id: u32,
    pub arena_type_2: u8,
    pub act3_fight_type: u8,
    pub act3_id: u32,
    pub arena_type_3: u8,
    pub content_entry_id: u32,
    pub order: u8,
    pub gil_reward: u16,
    pub allied_seals_reward: u16,
    pub tomestones_reward: u16,
}

impl Sheet for AOZContent {
    const SHEET_NAME: &'static str = "AOZContent";
}

impl FromExcelRow for AOZContent {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            standard_finish_time: single_row.columns.get(0).to_u16(),
            ideal_finish_time: single_row.columns.get(1).to_u16(),
            act1_fight_type: single_row.columns.get(2).to_u8(),
            act1_id: single_row.columns.get(3).to_u32(),
            arena_type_1: single_row.columns.get(4).to_u8(),
            act2_fight_type: single_row.columns.get(6).to_u8(),
            act2_id: single_row.columns.get(7).to_u32(),
            arena_type_2: single_row.columns.get(8).to_u8(),
            act3_fight_type: single_row.columns.get(10).to_u8(),
            act3_id: single_row.columns.get(11).to_u32(),
            arena_type_3: single_row.columns.get(12).to_u8(),
            content_entry_id: single_row.columns.get(14).to_u32(),
            order: single_row.columns.get(15).to_u8(),
            gil_reward: single_row.columns.get(16).to_u16(),
            allied_seals_reward: single_row.columns.get(17).to_u16(),
            tomestones_reward: single_row.columns.get(18).to_u16(),
        })
    }
}

