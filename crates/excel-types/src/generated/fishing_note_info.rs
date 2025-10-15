/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FishingNoteInfo {
    pub row_id: u32,
    pub size: u8,
    pub aquarium_water_id: u32,
    pub aquarium_water: RowRef<AquariumWater>,
    pub weather_restriction: u8,
    pub time_restriction: u8,
    pub special_conditions: u8,
    pub is_collectable: u8,
    pub item_id: u32,
    pub item_event_item: RowRef<EventItem>,
    pub item: RowRef<Item>,
}

impl Sheet for FishingNoteInfo {
    const SHEET_NAME: &'static str = "FishingNoteInfo";
}

impl FromExcelRow for FishingNoteInfo {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            size: single_row.columns.get(0).to_u8(),
            aquarium_water_id: single_row.columns.get(1).to_u32(),
            aquarium_water: RowRef::<AquariumWater>::from(single_row.columns.get(1).to_u32()),
            weather_restriction: single_row.columns.get(2).to_u8(),
            time_restriction: single_row.columns.get(3).to_u8(),
            special_conditions: single_row.columns.get(4).to_u8(),
            is_collectable: single_row.columns.get(5).to_u8(),
            item_id: single_row.columns.get(6).to_u32(),
            item_event_item: RowRef::<EventItem>::from(single_row.columns.get(6).to_u32()),
            item: RowRef::<Item>::from(single_row.columns.get(6).to_u32()),
        })
    }
}

