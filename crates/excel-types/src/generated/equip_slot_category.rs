/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct EquipSlotCategory {
    pub row_id: u32,
    pub main_hand: i8,
    pub off_hand: i8,
    pub head: i8,
    pub body: i8,
    pub gloves: i8,
    pub waist: i8,
    pub legs: i8,
    pub feet: i8,
    pub ears: i8,
    pub neck: i8,
    pub wrists: i8,
    pub finger_l: i8,
    pub finger_r: i8,
    pub soul_crystal: i8,
}

impl Sheet for EquipSlotCategory {
    const SHEET_NAME: &'static str = "EquipSlotCategory";
}

impl FromExcelRow for EquipSlotCategory {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            main_hand: single_row.columns.get(0).to_i8(),
            off_hand: single_row.columns.get(1).to_i8(),
            head: single_row.columns.get(2).to_i8(),
            body: single_row.columns.get(3).to_i8(),
            gloves: single_row.columns.get(4).to_i8(),
            waist: single_row.columns.get(5).to_i8(),
            legs: single_row.columns.get(6).to_i8(),
            feet: single_row.columns.get(7).to_i8(),
            ears: single_row.columns.get(8).to_i8(),
            neck: single_row.columns.get(9).to_i8(),
            wrists: single_row.columns.get(10).to_i8(),
            finger_l: single_row.columns.get(11).to_i8(),
            finger_r: single_row.columns.get(12).to_i8(),
            soul_crystal: single_row.columns.get(13).to_i8(),
        })
    }
}

