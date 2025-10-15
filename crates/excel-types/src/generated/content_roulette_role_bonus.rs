/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ContentRouletteRoleBonus {
    pub row_id: u32,
    pub item_reward_type_id: u32,
    pub item_reward_type: RowRef<Item>,
    pub reward_amount: u8,
}

impl Sheet for ContentRouletteRoleBonus {
    const SHEET_NAME: &'static str = "ContentRouletteRoleBonus";
}

impl FromExcelRow for ContentRouletteRoleBonus {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_reward_type_id: single_row.columns.get(6).to_u32(),
            item_reward_type: RowRef::<Item>::from(single_row.columns.get(6).to_u32()),
            reward_amount: single_row.columns.get(7).to_u8(),
        })
    }
}

