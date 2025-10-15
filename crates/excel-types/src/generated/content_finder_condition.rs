/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ContentFinderCondition {
    pub row_id: u32,
    pub short_code: String,
    pub territory_type_id: u32,
    pub territory_type: RowRef<TerritoryType>,
    pub content_link_type: u8,
    pub content_id: u32,
    pub pv_p: bool,
    pub accept_class_job_category_id: u32,
    pub accept_class_job_category: RowRef<ClassJobCategory>,
    pub content_member_type_id: u32,
    pub content_member_type: RowRef<ContentMemberType>,
    pub unlock_quest_id: u32,
    pub unlock_quest: RowRef<Quest>,
    pub class_job_level_required: u8,
    pub class_job_level_sync: u8,
    pub item_level_required: u16,
    pub item_level_sync: u16,
    pub allow_undersized: bool,
    pub allow_replacement: bool,
    pub allow_explorer_mode: bool,
    pub high_end_duty: bool,
    pub duty_recorder_allowed: bool,
    pub name: String,
    pub name_short: String,
    pub content_type_id: u32,
    pub content_type: RowRef<ContentType>,
    pub transient_key: u8,
    pub transient_id: u32,
    pub sort_key: u16,
    pub image_id: u32,
    pub icon_id: u32,
    pub leveling_roulette: bool,
    pub high_level_roulette: bool,
    pub msq_roulette: bool,
    pub guild_hest_roulette: bool,
    pub expert_roulette: bool,
    pub trial_roulette: bool,
    pub daily_frontline_challenge: bool,
    pub level_cap_roulette: bool,
    pub mentor_roulette: bool,
    pub alliance_roulette: bool,
    pub feast_team_roulette: bool,
    pub normal_raid_roulette: bool,
}

impl Sheet for ContentFinderCondition {
    const SHEET_NAME: &'static str = "ContentFinderCondition";
}

impl FromExcelRow for ContentFinderCondition {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            short_code: single_row.columns.get(0).to_owned_string(),
            territory_type_id: single_row.columns.get(1).to_u32(),
            territory_type: RowRef::<TerritoryType>::from(single_row.columns.get(1).to_u32()),
            content_link_type: single_row.columns.get(2).to_u8(),
            content_id: single_row.columns.get(3).to_u32(),
            pv_p: single_row.columns.get(4).to_bit(0),
            accept_class_job_category_id: single_row.columns.get(9).to_u32(),
            accept_class_job_category: RowRef::<ClassJobCategory>::from(single_row.columns.get(9).to_u32()),
            content_member_type_id: single_row.columns.get(10).to_u32(),
            content_member_type: RowRef::<ContentMemberType>::from(single_row.columns.get(10).to_u32()),
            unlock_quest_id: single_row.columns.get(14).to_u32(),
            unlock_quest: RowRef::<Quest>::from(single_row.columns.get(14).to_u32()),
            class_job_level_required: single_row.columns.get(17).to_u8(),
            class_job_level_sync: single_row.columns.get(18).to_u8(),
            item_level_required: single_row.columns.get(19).to_u16(),
            item_level_sync: single_row.columns.get(20).to_u16(),
            allow_undersized: single_row.columns.get(22).to_bit(3),
            allow_replacement: single_row.columns.get(25).to_bit(6),
            allow_explorer_mode: single_row.columns.get(27).to_bit(0),
            high_end_duty: single_row.columns.get(32).to_bit(4),
            duty_recorder_allowed: single_row.columns.get(38).to_bit(1),
            name: single_row.columns.get(43).to_owned_string(),
            name_short: single_row.columns.get(44).to_owned_string(),
            content_type_id: single_row.columns.get(45).to_u32(),
            content_type: RowRef::<ContentType>::from(single_row.columns.get(45).to_u32()),
            transient_key: single_row.columns.get(46).to_u8(),
            transient_id: single_row.columns.get(48).to_u32(),
            sort_key: single_row.columns.get(49).to_u16(),
            image_id: single_row.columns.get(50).to_u32(),
            icon_id: single_row.columns.get(51).to_u32(),
            leveling_roulette: single_row.columns.get(57).to_bool(),
            high_level_roulette: single_row.columns.get(58).to_bool(),
            msq_roulette: single_row.columns.get(59).to_bool(),
            guild_hest_roulette: single_row.columns.get(60).to_bool(),
            expert_roulette: single_row.columns.get(61).to_bool(),
            trial_roulette: single_row.columns.get(62).to_bool(),
            daily_frontline_challenge: single_row.columns.get(63).to_bool(),
            level_cap_roulette: single_row.columns.get(64).to_bool(),
            mentor_roulette: single_row.columns.get(65).to_bool(),
            alliance_roulette: single_row.columns.get(71).to_bool(),
            feast_team_roulette: single_row.columns.get(72).to_bool(),
            normal_raid_roulette: single_row.columns.get(73).to_bool(),
        })
    }
}

