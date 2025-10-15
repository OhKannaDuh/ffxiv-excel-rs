/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TripleTriad {
    pub row_id: u32,
    pub uses_regional_rules: bool,
    pub fee: u16,
    pub previous_quest_join: u8,
    pub start_time: u16,
    pub end_time: u16,
    pub default_talk_challenge_id: u32,
    pub default_talk_challenge: RowRef<DefaultTalk>,
    pub default_talk_unavailable_id: u32,
    pub default_talk_unavailable: RowRef<DefaultTalk>,
    pub default_talk_npc_win_id: u32,
    pub default_talk_npc_win: RowRef<DefaultTalk>,
    pub default_talk_draw_id: u32,
    pub default_talk_draw: RowRef<DefaultTalk>,
    pub default_talk_pc_win_id: u32,
    pub default_talk_pc_win: RowRef<DefaultTalk>,
}

impl Sheet for TripleTriad {
    const SHEET_NAME: &'static str = "TripleTriad";
}

impl FromExcelRow for TripleTriad {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            uses_regional_rules: single_row.columns.get(12).to_bit(0),
            fee: single_row.columns.get(13).to_u16(),
            previous_quest_join: single_row.columns.get(14).to_u8(),
            start_time: single_row.columns.get(18).to_u16(),
            end_time: single_row.columns.get(19).to_u16(),
            default_talk_challenge_id: single_row.columns.get(20).to_u32(),
            default_talk_challenge: RowRef::<DefaultTalk>::from(single_row.columns.get(20).to_u32()),
            default_talk_unavailable_id: single_row.columns.get(21).to_u32(),
            default_talk_unavailable: RowRef::<DefaultTalk>::from(single_row.columns.get(21).to_u32()),
            default_talk_npc_win_id: single_row.columns.get(22).to_u32(),
            default_talk_npc_win: RowRef::<DefaultTalk>::from(single_row.columns.get(22).to_u32()),
            default_talk_draw_id: single_row.columns.get(23).to_u32(),
            default_talk_draw: RowRef::<DefaultTalk>::from(single_row.columns.get(23).to_u32()),
            default_talk_pc_win_id: single_row.columns.get(24).to_u32(),
            default_talk_pc_win: RowRef::<DefaultTalk>::from(single_row.columns.get(24).to_u32()),
        })
    }
}

