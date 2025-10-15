/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DeepDungeonLayer {
    pub row_id: u32,
    pub deep_dungeon_id: u32,
    pub deep_dungeon: RowRef<DeepDungeon>,
    pub floor_set: u8,
    pub room_a_id: u32,
    pub room_a: RowRef<DeepDungeonMap5X>,
    pub room_b_id: u32,
    pub room_b: RowRef<DeepDungeonMap5X>,
    pub room_c_id: u32,
    pub room_c: RowRef<DeepDungeonMap5X>,
    pub wep_min_lv: u8,
    pub armour_min_lv: u8,
}

impl Sheet for DeepDungeonLayer {
    const SHEET_NAME: &'static str = "DeepDungeonLayer";
}

impl FromExcelRow for DeepDungeonLayer {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            deep_dungeon_id: single_row.columns.get(0).to_u32(),
            deep_dungeon: RowRef::<DeepDungeon>::from(single_row.columns.get(0).to_u32()),
            floor_set: single_row.columns.get(1).to_u8(),
            room_a_id: single_row.columns.get(2).to_u32(),
            room_a: RowRef::<DeepDungeonMap5X>::from(single_row.columns.get(2).to_u32()),
            room_b_id: single_row.columns.get(3).to_u32(),
            room_b: RowRef::<DeepDungeonMap5X>::from(single_row.columns.get(3).to_u32()),
            room_c_id: single_row.columns.get(4).to_u32(),
            room_c: RowRef::<DeepDungeonMap5X>::from(single_row.columns.get(4).to_u32()),
            wep_min_lv: single_row.columns.get(5).to_u8(),
            armour_min_lv: single_row.columns.get(6).to_u8(),
        })
    }
}

