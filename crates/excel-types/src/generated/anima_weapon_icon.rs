/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AnimaWeaponIcon {
    pub row_id: u32,
    pub hyperconductive_id: u32,
    pub reborn_id: u32,
    pub sharpened_id: u32,
    pub zodiac_id: u32,
    pub zodiac_lux_id: u32,
}

impl Sheet for AnimaWeaponIcon {
    const SHEET_NAME: &'static str = "AnimaWeaponIcon";
}

impl FromExcelRow for AnimaWeaponIcon {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            hyperconductive_id: single_row.columns.get(0).to_u32(),
            reborn_id: single_row.columns.get(1).to_u32(),
            sharpened_id: single_row.columns.get(2).to_u32(),
            zodiac_id: single_row.columns.get(3).to_u32(),
            zodiac_lux_id: single_row.columns.get(4).to_u32(),
        })
    }
}

