/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Aetheryte {
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
    pub aethernet_name_id: u32,
    pub aethernet_name: RowRef<PlaceName>,
    pub territory_id: u32,
    pub territory: RowRef<TerritoryType>,
    pub is_aetheryte: bool,
    pub aethernet_group: u8,
    pub invisible: bool,
    pub required_quest_id: u32,
    pub required_quest: RowRef<Quest>,
    pub map_id: u32,
    pub map: RowRef<Map>,
    pub aetherstream_x: i16,
    pub aetherstream_y: i16,
    pub order: u8,
}

impl Sheet for Aetheryte {
    const SHEET_NAME: &'static str = "Aetheryte";
}

impl FromExcelRow for Aetheryte {
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
            aethernet_name_id: single_row.columns.get(9).to_u32(),
            aethernet_name: RowRef::<PlaceName>::from(single_row.columns.get(9).to_u32()),
            territory_id: single_row.columns.get(10).to_u32(),
            territory: RowRef::<TerritoryType>::from(single_row.columns.get(10).to_u32()),
            is_aetheryte: single_row.columns.get(15).to_bit(0),
            aethernet_group: single_row.columns.get(18).to_u8(),
            invisible: single_row.columns.get(19).to_bit(1),
            required_quest_id: single_row.columns.get(20).to_u32(),
            required_quest: RowRef::<Quest>::from(single_row.columns.get(20).to_u32()),
            map_id: single_row.columns.get(21).to_u32(),
            map: RowRef::<Map>::from(single_row.columns.get(21).to_u32()),
            aetherstream_x: single_row.columns.get(22).to_i16(),
            aetherstream_y: single_row.columns.get(23).to_i16(),
            order: single_row.columns.get(24).to_u8(),
        })
    }
}

