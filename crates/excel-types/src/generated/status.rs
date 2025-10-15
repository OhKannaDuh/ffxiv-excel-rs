/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Status {
    pub row_id: u32,
    pub name: String,
    pub description: String,
    pub icon_id: u32,
    pub max_stacks: u8,
    pub class_job_category_id: u32,
    pub class_job_category: RowRef<ClassJobCategory>,
    pub status_category: u8,
    pub hit_effect_id: u32,
    pub hit_effect: RowRef<StatusHitEffect>,
    pub vfx_id: u32,
    pub vfx: RowRef<StatusLoopVFX>,
    pub lock_movement: bool,
    pub lock_actions: bool,
    pub lock_control: bool,
    pub transfiguration: bool,
    pub is_gaze: bool,
    pub can_dispel: bool,
    pub inflicted_by_actor: bool,
    pub is_permanent: bool,
    pub party_list_priority: u8,
    pub can_increase_rewards: u8,
    pub param_modifier: i32,
    pub param_effect: u8,
    pub can_status_off: bool,
    pub log: u16,
    pub is_fc_buff: bool,
    pub invisibility: bool,
    pub target_type: u8,
    pub flags: u8,
}

impl Sheet for Status {
    const SHEET_NAME: &'static str = "Status";
}

impl FromExcelRow for Status {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            description: single_row.columns.get(1).to_owned_string(),
            icon_id: single_row.columns.get(2).to_u32(),
            max_stacks: single_row.columns.get(4).to_u8(),
            class_job_category_id: single_row.columns.get(5).to_u32(),
            class_job_category: RowRef::<ClassJobCategory>::from(single_row.columns.get(5).to_u32()),
            status_category: single_row.columns.get(6).to_u8(),
            hit_effect_id: single_row.columns.get(7).to_u32(),
            hit_effect: RowRef::<StatusHitEffect>::from(single_row.columns.get(7).to_u32()),
            vfx_id: single_row.columns.get(8).to_u32(),
            vfx: RowRef::<StatusLoopVFX>::from(single_row.columns.get(8).to_u32()),
            lock_movement: single_row.columns.get(9).to_bit(0),
            lock_actions: single_row.columns.get(11).to_bit(2),
            lock_control: single_row.columns.get(12).to_bit(3),
            transfiguration: single_row.columns.get(13).to_bit(4),
            is_gaze: single_row.columns.get(14).to_bit(5),
            can_dispel: single_row.columns.get(15).to_bit(6),
            inflicted_by_actor: single_row.columns.get(16).to_bit(7),
            is_permanent: single_row.columns.get(17).to_bit(0),
            party_list_priority: single_row.columns.get(18).to_u8(),
            can_increase_rewards: single_row.columns.get(19).to_u8(),
            param_modifier: single_row.columns.get(22).to_i32(),
            param_effect: single_row.columns.get(23).to_u8(),
            can_status_off: single_row.columns.get(24).to_bit(3),
            log: single_row.columns.get(25).to_u16(),
            is_fc_buff: single_row.columns.get(26).to_bit(4),
            invisibility: single_row.columns.get(28).to_bit(5),
            target_type: single_row.columns.get(29).to_u8(),
            flags: single_row.columns.get(30).to_u8(),
        })
    }
}

