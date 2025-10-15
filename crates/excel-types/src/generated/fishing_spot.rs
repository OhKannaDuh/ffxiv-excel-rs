/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FishingSpot {
    pub row_id: u32,
    pub gathering_level: u8,
    pub big_fish_on_reach: String,
    pub big_fish_on_end: String,
    pub fishing_spot_category: u8,
    pub rare: bool,
    pub territory_type_id: u32,
    pub territory_type: RowRef<TerritoryType>,
    pub place_name_main_id: u32,
    pub place_name_main: RowRef<PlaceName>,
    pub place_name_sub_id: u32,
    pub place_name_sub: RowRef<PlaceName>,
    pub x: i16,
    pub z: i16,
    pub radius: u16,
    pub place_name_id: u32,
    pub place_name: RowRef<PlaceName>,
    pub order: u16,
}

impl Sheet for FishingSpot {
    const SHEET_NAME: &'static str = "FishingSpot";
}

impl FromExcelRow for FishingSpot {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            gathering_level: single_row.columns.get(0).to_u8(),
            big_fish_on_reach: single_row.columns.get(1).to_owned_string(),
            big_fish_on_end: single_row.columns.get(2).to_owned_string(),
            fishing_spot_category: single_row.columns.get(4).to_u8(),
            rare: single_row.columns.get(5).to_bit(0),
            territory_type_id: single_row.columns.get(6).to_u32(),
            territory_type: RowRef::<TerritoryType>::from(single_row.columns.get(6).to_u32()),
            place_name_main_id: single_row.columns.get(7).to_u32(),
            place_name_main: RowRef::<PlaceName>::from(single_row.columns.get(7).to_u32()),
            place_name_sub_id: single_row.columns.get(8).to_u32(),
            place_name_sub: RowRef::<PlaceName>::from(single_row.columns.get(8).to_u32()),
            x: single_row.columns.get(9).to_i16(),
            z: single_row.columns.get(10).to_i16(),
            radius: single_row.columns.get(11).to_u16(),
            place_name_id: single_row.columns.get(23).to_u32(),
            place_name: RowRef::<PlaceName>::from(single_row.columns.get(23).to_u32()),
            order: single_row.columns.get(24).to_u16(),
        })
    }
}

