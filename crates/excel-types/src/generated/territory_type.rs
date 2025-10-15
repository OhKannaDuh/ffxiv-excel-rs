/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TerritoryType {
    pub row_id: u32,
    pub name: String,
    pub bg: String,
    pub battalion_mode: u8,
    pub place_name_region_id: u32,
    pub place_name_region: RowRef<PlaceName>,
    pub place_name_zone_id: u32,
    pub place_name_zone: RowRef<PlaceName>,
    pub place_name_id: u32,
    pub place_name: RowRef<PlaceName>,
    pub map_id: u32,
    pub map: RowRef<Map>,
    pub loading_image_id: u32,
    pub exclusive_type: u8,
    pub territory_intended_use_id: u32,
    pub content_finder_condition_id: u32,
    pub content_finder_condition: RowRef<ContentFinderCondition>,
    pub weather_rate: u8,
    pub pc_search: bool,
    pub stealth: bool,
    pub mount: bool,
    pub bgm_id: u32,
    pub bgm: RowRef<BGM>,
    pub bgm_situation: RowRef<BGMSituation>,
    pub place_name_region_icon_id: u32,
    pub place_name_icon_id: u32,
    pub array_event_handler_id: u32,
    pub array_event_handler: RowRef<ArrayEventHandler>,
    pub quest_battle_id: u32,
    pub quest_battle: RowRef<QuestBattle>,
    pub aetheryte_id: u32,
    pub aetheryte: RowRef<Aetheryte>,
    pub fixed_time: i32,
    pub resident: u16,
    pub achievement_index: i8,
    pub is_pvp_zone: bool,
    pub ex_version_id: u32,
    pub ex_version: RowRef<ExVersion>,
    pub mount_speed_id: u32,
    pub mount_speed: RowRef<MountSpeed>,
}

impl Sheet for TerritoryType {
    const SHEET_NAME: &'static str = "TerritoryType";
}

impl FromExcelRow for TerritoryType {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            bg: single_row.columns.get(1).to_owned_string(),
            battalion_mode: single_row.columns.get(2).to_u8(),
            place_name_region_id: single_row.columns.get(3).to_u32(),
            place_name_region: RowRef::<PlaceName>::from(single_row.columns.get(3).to_u32()),
            place_name_zone_id: single_row.columns.get(4).to_u32(),
            place_name_zone: RowRef::<PlaceName>::from(single_row.columns.get(4).to_u32()),
            place_name_id: single_row.columns.get(5).to_u32(),
            place_name: RowRef::<PlaceName>::from(single_row.columns.get(5).to_u32()),
            map_id: single_row.columns.get(6).to_u32(),
            map: RowRef::<Map>::from(single_row.columns.get(6).to_u32()),
            loading_image_id: single_row.columns.get(7).to_u32(),
            exclusive_type: single_row.columns.get(8).to_u8(),
            territory_intended_use_id: single_row.columns.get(9).to_u32(),
            content_finder_condition_id: single_row.columns.get(10).to_u32(),
            content_finder_condition: RowRef::<ContentFinderCondition>::from(single_row.columns.get(10).to_u32()),
            weather_rate: single_row.columns.get(12).to_u8(),
            pc_search: single_row.columns.get(15).to_bit(2),
            stealth: single_row.columns.get(16).to_bit(3),
            mount: single_row.columns.get(17).to_bit(4),
            bgm_id: single_row.columns.get(19).to_u32(),
            bgm: RowRef::<BGM>::from(single_row.columns.get(19).to_u32()),
            bgm_situation: RowRef::<BGMSituation>::from(single_row.columns.get(19).to_u32()),
            place_name_region_icon_id: single_row.columns.get(20).to_u32(),
            place_name_icon_id: single_row.columns.get(21).to_u32(),
            array_event_handler_id: single_row.columns.get(22).to_u32(),
            array_event_handler: RowRef::<ArrayEventHandler>::from(single_row.columns.get(22).to_u32()),
            quest_battle_id: single_row.columns.get(23).to_u32(),
            quest_battle: RowRef::<QuestBattle>::from(single_row.columns.get(23).to_u32()),
            aetheryte_id: single_row.columns.get(24).to_u32(),
            aetheryte: RowRef::<Aetheryte>::from(single_row.columns.get(24).to_u32()),
            fixed_time: single_row.columns.get(25).to_i32(),
            resident: single_row.columns.get(26).to_u16(),
            achievement_index: single_row.columns.get(27).to_i8(),
            is_pvp_zone: single_row.columns.get(28).to_bit(6),
            ex_version_id: single_row.columns.get(29).to_u32(),
            ex_version: RowRef::<ExVersion>::from(single_row.columns.get(29).to_u32()),
            mount_speed_id: single_row.columns.get(33).to_u32(),
            mount_speed: RowRef::<MountSpeed>::from(single_row.columns.get(33).to_u32()),
        })
    }
}

