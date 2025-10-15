/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct InstanceContent {
    pub row_id: u32,
    pub instance_content_type_id: u32,
    pub week_restriction: u8,
    pub time_limit_min: u16,
    pub bgm_id: u32,
    pub bgm: RowRef<BGM>,
    pub win_bgm_id: u32,
    pub win_bgm: RowRef<BGM>,
    pub cutscene_id: u32,
    pub cutscene: RowRef<Cutscene>,
    pub lgb_event_range: u32,
    pub order: u16,
    pub colosseum_id: u32,
    pub instance_content_text_data_boss_start_id: u32,
    pub instance_content_text_data_boss_start: RowRef<InstanceContentTextData>,
    pub instance_content_text_data_boss_end_id: u32,
    pub instance_content_text_data_boss_end: RowRef<InstanceContentTextData>,
    pub b_npc_base_boss_id: u32,
    pub b_npc_base_boss: RowRef<BNpcBase>,
    pub instance_content_text_data_objective_start_id: u32,
    pub instance_content_text_data_objective_start: RowRef<InstanceContentTextData>,
    pub instance_content_text_data_objective_end_id: u32,
    pub instance_content_text_data_objective_end: RowRef<InstanceContentTextData>,
    pub sort_key: u16,
    pub new_player_bonus_gil: u32,
    pub new_player_bonus_exp: u32,
    pub new_player_bonus_a: u16,
    pub new_player_bonus_b: u16,
    pub final_boss_exp: u32,
    pub final_boss_currency_a: u16,
    pub final_boss_currency_b: u16,
    pub final_boss_currency_c: u16,
    pub instance_clear_exp: u32,
    pub instance_clear_gil: u32,
    pub instance_content_reward_item_id: u32,
    pub instance_content_buff_id: u32,
    pub instance_content_buff: RowRef<InstanceContentBuff>,
    pub req_instance_id: u32,
    pub req_instance: RowRef<InstanceContent>,
    pub party_condition: u32,
}

impl Sheet for InstanceContent {
    const SHEET_NAME: &'static str = "InstanceContent";
}

impl FromExcelRow for InstanceContent {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            instance_content_type_id: single_row.columns.get(0).to_u32(),
            week_restriction: single_row.columns.get(1).to_u8(),
            time_limit_min: single_row.columns.get(2).to_u16(),
            bgm_id: single_row.columns.get(4).to_u32(),
            bgm: RowRef::<BGM>::from(single_row.columns.get(4).to_u32()),
            win_bgm_id: single_row.columns.get(5).to_u32(),
            win_bgm: RowRef::<BGM>::from(single_row.columns.get(5).to_u32()),
            cutscene_id: single_row.columns.get(6).to_u32(),
            cutscene: RowRef::<Cutscene>::from(single_row.columns.get(6).to_u32()),
            lgb_event_range: single_row.columns.get(7).to_u32(),
            order: single_row.columns.get(8).to_u16(),
            colosseum_id: single_row.columns.get(9).to_u32(),
            instance_content_text_data_boss_start_id: single_row.columns.get(11).to_u32(),
            instance_content_text_data_boss_start: RowRef::<InstanceContentTextData>::from(single_row.columns.get(11).to_u32()),
            instance_content_text_data_boss_end_id: single_row.columns.get(12).to_u32(),
            instance_content_text_data_boss_end: RowRef::<InstanceContentTextData>::from(single_row.columns.get(12).to_u32()),
            b_npc_base_boss_id: single_row.columns.get(13).to_u32(),
            b_npc_base_boss: RowRef::<BNpcBase>::from(single_row.columns.get(13).to_u32()),
            instance_content_text_data_objective_start_id: single_row.columns.get(14).to_u32(),
            instance_content_text_data_objective_start: RowRef::<InstanceContentTextData>::from(single_row.columns.get(14).to_u32()),
            instance_content_text_data_objective_end_id: single_row.columns.get(15).to_u32(),
            instance_content_text_data_objective_end: RowRef::<InstanceContentTextData>::from(single_row.columns.get(15).to_u32()),
            sort_key: single_row.columns.get(16).to_u16(),
            new_player_bonus_gil: single_row.columns.get(17).to_u32(),
            new_player_bonus_exp: single_row.columns.get(18).to_u32(),
            new_player_bonus_a: single_row.columns.get(19).to_u16(),
            new_player_bonus_b: single_row.columns.get(20).to_u16(),
            final_boss_exp: single_row.columns.get(21).to_u32(),
            final_boss_currency_a: single_row.columns.get(23).to_u16(),
            final_boss_currency_b: single_row.columns.get(24).to_u16(),
            final_boss_currency_c: single_row.columns.get(25).to_u16(),
            instance_clear_exp: single_row.columns.get(46).to_u32(),
            instance_clear_gil: single_row.columns.get(47).to_u32(),
            instance_content_reward_item_id: single_row.columns.get(48).to_u32(),
            instance_content_buff_id: single_row.columns.get(51).to_u32(),
            instance_content_buff: RowRef::<InstanceContentBuff>::from(single_row.columns.get(51).to_u32()),
            req_instance_id: single_row.columns.get(53).to_u32(),
            req_instance: RowRef::<InstanceContent>::from(single_row.columns.get(53).to_u32()),
            party_condition: single_row.columns.get(54).to_u32(),
        })
    }
}

