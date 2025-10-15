/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AnimaWeapon5TradeItem {
    pub row_id: u32,
    pub crystal_sand_id: u32,
    pub crystal_sand: RowRef<Item>,
    pub qty: u8,
    pub category_id: u32,
    pub category: RowRef<AnimaWeapon5PatternGroup>,
}

impl Sheet for AnimaWeapon5TradeItem {
    const SHEET_NAME: &'static str = "AnimaWeapon5TradeItem";
}

impl FromExcelRow for AnimaWeapon5TradeItem {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            crystal_sand_id: single_row.columns.get(1).to_u32(),
            crystal_sand: RowRef::<Item>::from(single_row.columns.get(1).to_u32()),
            qty: single_row.columns.get(2).to_u8(),
            category_id: single_row.columns.get(27).to_u32(),
            category: RowRef::<AnimaWeapon5PatternGroup>::from(single_row.columns.get(27).to_u32()),
        })
    }
}

