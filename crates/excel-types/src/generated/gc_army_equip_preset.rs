/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GcArmyEquipPreset {
    pub row_id: u32,
    pub main_hand_id: u32,
    pub main_hand: RowRef<Item>,
    pub off_hand_id: u32,
    pub off_hand: RowRef<Item>,
    pub head_id: u32,
    pub head: RowRef<Item>,
    pub body_id: u32,
    pub body: RowRef<Item>,
    pub gloves_id: u32,
    pub gloves: RowRef<Item>,
    pub legs_id: u32,
    pub legs: RowRef<Item>,
    pub feet_id: u32,
    pub feet: RowRef<Item>,
}

impl Sheet for GcArmyEquipPreset {
    const SHEET_NAME: &'static str = "GcArmyEquipPreset";
}

impl FromExcelRow for GcArmyEquipPreset {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            main_hand_id: single_row.columns.get(0).to_u32(),
            main_hand: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            off_hand_id: single_row.columns.get(1).to_u32(),
            off_hand: RowRef::<Item>::from(single_row.columns.get(1).to_u32()),
            head_id: single_row.columns.get(2).to_u32(),
            head: RowRef::<Item>::from(single_row.columns.get(2).to_u32()),
            body_id: single_row.columns.get(3).to_u32(),
            body: RowRef::<Item>::from(single_row.columns.get(3).to_u32()),
            gloves_id: single_row.columns.get(4).to_u32(),
            gloves: RowRef::<Item>::from(single_row.columns.get(4).to_u32()),
            legs_id: single_row.columns.get(5).to_u32(),
            legs: RowRef::<Item>::from(single_row.columns.get(5).to_u32()),
            feet_id: single_row.columns.get(6).to_u32(),
            feet: RowRef::<Item>::from(single_row.columns.get(6).to_u32()),
        })
    }
}

