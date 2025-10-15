/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DeepDungeon {
    pub row_id: u32,
    pub aetherpool_arm_id: u32,
    pub aetherpool_arm: RowRef<DeepDungeonEquipment>,
    pub aetherpool_armor_id: u32,
    pub aetherpool_armor: RowRef<DeepDungeonEquipment>,
    pub deep_dungeon_type: u8,
    pub name: u8,
    pub content_finder_condition_start_id: u32,
    pub content_finder_condition_start: RowRef<ContentFinderCondition>,
}

impl Sheet for DeepDungeon {
    const SHEET_NAME: &'static str = "DeepDungeon";
}

impl FromExcelRow for DeepDungeon {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            aetherpool_arm_id: single_row.columns.get(0).to_u32(),
            aetherpool_arm: RowRef::<DeepDungeonEquipment>::from(single_row.columns.get(0).to_u32()),
            aetherpool_armor_id: single_row.columns.get(1).to_u32(),
            aetherpool_armor: RowRef::<DeepDungeonEquipment>::from(single_row.columns.get(1).to_u32()),
            deep_dungeon_type: single_row.columns.get(18).to_u8(),
            name: single_row.columns.get(23).to_u8(),
            content_finder_condition_start_id: single_row.columns.get(24).to_u32(),
            content_finder_condition_start: RowRef::<ContentFinderCondition>::from(single_row.columns.get(24).to_u32()),
        })
    }
}

