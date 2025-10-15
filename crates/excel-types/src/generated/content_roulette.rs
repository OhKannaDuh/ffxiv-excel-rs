/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ContentRoulette {
    pub row_id: u32,
    pub name: String,
    pub category: String,
    pub description: String,
    pub duty_type: String,
    pub is_gold_saucer: bool,
    pub is_in_duty_finder: bool,
    pub open_rule_id: u32,
    pub open_rule: RowRef<ContentRouletteOpenRule>,
    pub is_pv_p: bool,
    pub required_level: u8,
    pub item_level_required: u16,
    pub icon_id: u32,
    pub content_roulette_role_bonus_id: u32,
    pub content_roulette_role_bonus: RowRef<ContentRouletteRoleBonus>,
    pub reward_tome_a: u16,
    pub reward_tome_b: u16,
    pub reward_tome_c: u16,
    pub sort_key: u8,
    pub content_member_type_id: u32,
    pub content_member_type: RowRef<ContentMemberType>,
    pub require_all_duties: bool,
    pub content_roulette_open_rule: bool,
    pub instance_content_id: u32,
    pub instance_content: RowRef<InstanceContent>,
}

impl Sheet for ContentRoulette {
    const SHEET_NAME: &'static str = "ContentRoulette";
}

impl FromExcelRow for ContentRoulette {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            category: single_row.columns.get(1).to_owned_string(),
            description: single_row.columns.get(3).to_owned_string(),
            duty_type: single_row.columns.get(4).to_owned_string(),
            is_gold_saucer: single_row.columns.get(7).to_bit(0),
            is_in_duty_finder: single_row.columns.get(8).to_bit(1),
            open_rule_id: single_row.columns.get(9).to_u32(),
            open_rule: RowRef::<ContentRouletteOpenRule>::from(single_row.columns.get(9).to_u32()),
            is_pv_p: single_row.columns.get(10).to_bit(2),
            required_level: single_row.columns.get(11).to_u8(),
            item_level_required: single_row.columns.get(13).to_u16(),
            icon_id: single_row.columns.get(16).to_u32(),
            content_roulette_role_bonus_id: single_row.columns.get(17).to_u32(),
            content_roulette_role_bonus: RowRef::<ContentRouletteRoleBonus>::from(single_row.columns.get(17).to_u32()),
            reward_tome_a: single_row.columns.get(18).to_u16(),
            reward_tome_b: single_row.columns.get(19).to_u16(),
            reward_tome_c: single_row.columns.get(20).to_u16(),
            sort_key: single_row.columns.get(24).to_u8(),
            content_member_type_id: single_row.columns.get(26).to_u32(),
            content_member_type: RowRef::<ContentMemberType>::from(single_row.columns.get(26).to_u32()),
            require_all_duties: single_row.columns.get(37).to_bit(7),
            content_roulette_open_rule: single_row.columns.get(39).to_bit(1),
            instance_content_id: single_row.columns.get(40).to_u32(),
            instance_content: RowRef::<InstanceContent>::from(single_row.columns.get(40).to_u32()),
        })
    }
}

