/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HousingPreset {
    pub row_id: u32,
    pub singular: String,
    pub adjective: i8,
    pub plural: String,
    pub possessive_pronoun: i8,
    pub starts_with_vowel: i8,
    pub pronoun: i8,
    pub article: i8,
    pub place_name_id: u32,
    pub place_name: RowRef<PlaceName>,
    pub housing_size: u8,
    pub exterior_roof_id: u32,
    pub exterior_roof: RowRef<Item>,
    pub exterior_wall_id: u32,
    pub exterior_wall: RowRef<Item>,
    pub exterior_window_id: u32,
    pub exterior_window: RowRef<Item>,
    pub exterior_door_id: u32,
    pub exterior_door: RowRef<Item>,
    pub interior_wall_id: u32,
    pub interior_wall: RowRef<Item>,
    pub interior_flooring_id: u32,
    pub interior_flooring: RowRef<Item>,
    pub interior_lighting_id: u32,
    pub interior_lighting: RowRef<Item>,
    pub other_floor_wall_id: u32,
    pub other_floor_wall: RowRef<Item>,
    pub other_floor_flooring_id: u32,
    pub other_floor_flooring: RowRef<Item>,
    pub other_floor_lighting_id: u32,
    pub other_floor_lighting: RowRef<Item>,
    pub basement_wall_id: u32,
    pub basement_wall: RowRef<Item>,
    pub basement_flooring_id: u32,
    pub basement_flooring: RowRef<Item>,
    pub basement_lighting_id: u32,
    pub basement_lighting: RowRef<Item>,
    pub mansion_lighting_id: u32,
    pub mansion_lighting: RowRef<Item>,
}

impl Sheet for HousingPreset {
    const SHEET_NAME: &'static str = "HousingPreset";
}

impl FromExcelRow for HousingPreset {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            singular: single_row.columns.get(0).to_owned_string(),
            adjective: single_row.columns.get(1).to_i8(),
            plural: single_row.columns.get(2).to_owned_string(),
            possessive_pronoun: single_row.columns.get(3).to_i8(),
            starts_with_vowel: single_row.columns.get(4).to_i8(),
            pronoun: single_row.columns.get(6).to_i8(),
            article: single_row.columns.get(7).to_i8(),
            place_name_id: single_row.columns.get(8).to_u32(),
            place_name: RowRef::<PlaceName>::from(single_row.columns.get(8).to_u32()),
            housing_size: single_row.columns.get(9).to_u8(),
            exterior_roof_id: single_row.columns.get(10).to_u32(),
            exterior_roof: RowRef::<Item>::from(single_row.columns.get(10).to_u32()),
            exterior_wall_id: single_row.columns.get(11).to_u32(),
            exterior_wall: RowRef::<Item>::from(single_row.columns.get(11).to_u32()),
            exterior_window_id: single_row.columns.get(12).to_u32(),
            exterior_window: RowRef::<Item>::from(single_row.columns.get(12).to_u32()),
            exterior_door_id: single_row.columns.get(13).to_u32(),
            exterior_door: RowRef::<Item>::from(single_row.columns.get(13).to_u32()),
            interior_wall_id: single_row.columns.get(14).to_u32(),
            interior_wall: RowRef::<Item>::from(single_row.columns.get(14).to_u32()),
            interior_flooring_id: single_row.columns.get(15).to_u32(),
            interior_flooring: RowRef::<Item>::from(single_row.columns.get(15).to_u32()),
            interior_lighting_id: single_row.columns.get(16).to_u32(),
            interior_lighting: RowRef::<Item>::from(single_row.columns.get(16).to_u32()),
            other_floor_wall_id: single_row.columns.get(17).to_u32(),
            other_floor_wall: RowRef::<Item>::from(single_row.columns.get(17).to_u32()),
            other_floor_flooring_id: single_row.columns.get(18).to_u32(),
            other_floor_flooring: RowRef::<Item>::from(single_row.columns.get(18).to_u32()),
            other_floor_lighting_id: single_row.columns.get(19).to_u32(),
            other_floor_lighting: RowRef::<Item>::from(single_row.columns.get(19).to_u32()),
            basement_wall_id: single_row.columns.get(20).to_u32(),
            basement_wall: RowRef::<Item>::from(single_row.columns.get(20).to_u32()),
            basement_flooring_id: single_row.columns.get(21).to_u32(),
            basement_flooring: RowRef::<Item>::from(single_row.columns.get(21).to_u32()),
            basement_lighting_id: single_row.columns.get(22).to_u32(),
            basement_lighting: RowRef::<Item>::from(single_row.columns.get(22).to_u32()),
            mansion_lighting_id: single_row.columns.get(23).to_u32(),
            mansion_lighting: RowRef::<Item>::from(single_row.columns.get(23).to_u32()),
        })
    }
}

