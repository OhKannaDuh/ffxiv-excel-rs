/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CollectablesShopRewardItem {
    pub row_id: u32,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub reward_low: u8,
    pub reward_mid: u8,
    pub reward_high: u8,
}

impl Sheet for CollectablesShopRewardItem {
    const SHEET_NAME: &'static str = "CollectablesShopRewardItem";
}

impl FromExcelRow for CollectablesShopRewardItem {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_id: single_row.columns.get(0).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            reward_low: single_row.columns.get(2).to_u8(),
            reward_mid: single_row.columns.get(3).to_u8(),
            reward_high: single_row.columns.get(4).to_u8(),
        })
    }
}

