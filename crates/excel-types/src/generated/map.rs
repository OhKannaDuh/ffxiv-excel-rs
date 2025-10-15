/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Map {
    pub row_id: u32,
    pub map_condition_id: u32,
    pub map_condition: RowRef<MapCondition>,
    pub priority_category_ui: u8,
    pub priority_ui: u8,
    pub map_index: i8,
    pub hierarchy: u8,
    pub map_marker_range: u16,
    pub id: String,
    pub size_factor: u16,
    pub offset_x: i16,
    pub offset_y: i16,
    pub place_name_region_id: u32,
    pub place_name_region: RowRef<PlaceName>,
    pub place_name_id: u32,
    pub place_name: RowRef<PlaceName>,
    pub place_name_sub_id: u32,
    pub place_name_sub: RowRef<PlaceName>,
    pub discovery_index: u8,
    pub discovery_flag: i16,
    pub territory_type_id: u32,
    pub territory_type: RowRef<TerritoryType>,
    pub discovery_array_byte: u16,
    pub is_event: bool,
}

impl Sheet for Map {
    const SHEET_NAME: &'static str = "Map";
}

impl FromExcelRow for Map {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            map_condition_id: single_row.columns.get(0).to_u32(),
            map_condition: RowRef::<MapCondition>::from(single_row.columns.get(0).to_u32()),
            priority_category_ui: single_row.columns.get(1).to_u8(),
            priority_ui: single_row.columns.get(2).to_u8(),
            map_index: single_row.columns.get(3).to_i8(),
            hierarchy: single_row.columns.get(4).to_u8(),
            map_marker_range: single_row.columns.get(5).to_u16(),
            id: single_row.columns.get(6).to_owned_string(),
            size_factor: single_row.columns.get(7).to_u16(),
            offset_x: single_row.columns.get(8).to_i16(),
            offset_y: single_row.columns.get(9).to_i16(),
            place_name_region_id: single_row.columns.get(10).to_u32(),
            place_name_region: RowRef::<PlaceName>::from(single_row.columns.get(10).to_u32()),
            place_name_id: single_row.columns.get(11).to_u32(),
            place_name: RowRef::<PlaceName>::from(single_row.columns.get(11).to_u32()),
            place_name_sub_id: single_row.columns.get(12).to_u32(),
            place_name_sub: RowRef::<PlaceName>::from(single_row.columns.get(12).to_u32()),
            discovery_index: single_row.columns.get(13).to_u8(),
            discovery_flag: single_row.columns.get(14).to_i16(),
            territory_type_id: single_row.columns.get(15).to_u32(),
            territory_type: RowRef::<TerritoryType>::from(single_row.columns.get(15).to_u32()),
            discovery_array_byte: single_row.columns.get(16).to_u16(),
            is_event: single_row.columns.get(17).to_bit(0),
        })
    }
}

