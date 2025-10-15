/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Fate {
    pub row_id: u32,
    pub name: String,
    pub description: String,
    pub objective: String,
    pub eureka_fate: u8,
    pub rule: u8,
    pub fate_rule_ex_id: u32,
    pub location: u32,
    pub class_job_level: u8,
    pub class_job_level_max: u8,
    pub event_item_id: u32,
    pub event_item: RowRef<EventItem>,
    pub icon_objective_id: u32,
    pub icon_map_id: u32,
    pub icon_inactive_map_id: u32,
    pub music_id: u32,
    pub music: RowRef<BGM>,
    pub lgb_guard_npc_location: u32,
    pub screen_image_accept_id: u32,
    pub screen_image_accept: RowRef<ScreenImage>,
    pub screen_image_complete_id: u32,
    pub screen_image_complete: RowRef<ScreenImage>,
    pub screen_image_failed_id: u32,
    pub screen_image_failed: RowRef<ScreenImage>,
    pub required_quest_id: u32,
    pub required_quest: RowRef<Quest>,
    pub special_fate: bool,
    pub given_status_id: u32,
    pub given_status: RowRef<Status>,
    pub advent_event: bool,
    pub moon_faire_event: bool,
    pub fate_chain: u32,
    pub array_index_id: u32,
    pub array_index: RowRef<ArrayEventHandler>,
    pub req_event_item_id: u32,
    pub req_event_item: RowRef<EventItem>,
    pub turn_in_event_item_id: u32,
    pub turn_in_event_item: RowRef<EventItem>,
}

impl Sheet for Fate {
    const SHEET_NAME: &'static str = "Fate";
}

impl FromExcelRow for Fate {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            description: single_row.columns.get(1).to_owned_string(),
            objective: single_row.columns.get(2).to_owned_string(),
            eureka_fate: single_row.columns.get(6).to_u8(),
            rule: single_row.columns.get(7).to_u8(),
            fate_rule_ex_id: single_row.columns.get(8).to_u32(),
            location: single_row.columns.get(9).to_u32(),
            class_job_level: single_row.columns.get(10).to_u8(),
            class_job_level_max: single_row.columns.get(11).to_u8(),
            event_item_id: single_row.columns.get(12).to_u32(),
            event_item: RowRef::<EventItem>::from(single_row.columns.get(12).to_u32()),
            icon_objective_id: single_row.columns.get(16).to_u32(),
            icon_map_id: single_row.columns.get(17).to_u32(),
            icon_inactive_map_id: single_row.columns.get(18).to_u32(),
            music_id: single_row.columns.get(19).to_u32(),
            music: RowRef::<BGM>::from(single_row.columns.get(19).to_u32()),
            lgb_guard_npc_location: single_row.columns.get(20).to_u32(),
            screen_image_accept_id: single_row.columns.get(21).to_u32(),
            screen_image_accept: RowRef::<ScreenImage>::from(single_row.columns.get(21).to_u32()),
            screen_image_complete_id: single_row.columns.get(22).to_u32(),
            screen_image_complete: RowRef::<ScreenImage>::from(single_row.columns.get(22).to_u32()),
            screen_image_failed_id: single_row.columns.get(23).to_u32(),
            screen_image_failed: RowRef::<ScreenImage>::from(single_row.columns.get(23).to_u32()),
            required_quest_id: single_row.columns.get(25).to_u32(),
            required_quest: RowRef::<Quest>::from(single_row.columns.get(25).to_u32()),
            special_fate: single_row.columns.get(26).to_bit(0),
            given_status_id: single_row.columns.get(28).to_u32(),
            given_status: RowRef::<Status>::from(single_row.columns.get(28).to_u32()),
            advent_event: single_row.columns.get(30).to_bit(2),
            moon_faire_event: single_row.columns.get(31).to_bit(3),
            fate_chain: single_row.columns.get(33).to_u32(),
            array_index_id: single_row.columns.get(36).to_u32(),
            array_index: RowRef::<ArrayEventHandler>::from(single_row.columns.get(36).to_u32()),
            req_event_item_id: single_row.columns.get(38).to_u32(),
            req_event_item: RowRef::<EventItem>::from(single_row.columns.get(38).to_u32()),
            turn_in_event_item_id: single_row.columns.get(39).to_u32(),
            turn_in_event_item: RowRef::<EventItem>::from(single_row.columns.get(39).to_u32()),
        })
    }
}

