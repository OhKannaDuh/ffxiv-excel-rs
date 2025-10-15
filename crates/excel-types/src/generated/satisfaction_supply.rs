/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct SatisfactionSupply {
    pub row_id: u32,
    pub slot: u8,
    pub probability_percent: u8,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub collectability_low: u16,
    pub collectability_mid: u16,
    pub collectability_high: u16,
    pub reward_id: u32,
    pub reward: RowRef<SatisfactionSupplyReward>,
}

impl Sheet for SatisfactionSupply {
    const SHEET_NAME: &'static str = "SatisfactionSupply";
}

impl FromExcelRow for SatisfactionSupply {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            slot: single_row.columns.get(0).to_u8(),
            probability_percent: single_row.columns.get(1).to_u8(),
            item_id: single_row.columns.get(2).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(2).to_u32()),
            collectability_low: single_row.columns.get(3).to_u16(),
            collectability_mid: single_row.columns.get(4).to_u16(),
            collectability_high: single_row.columns.get(5).to_u16(),
            reward_id: single_row.columns.get(6).to_u32(),
            reward: RowRef::<SatisfactionSupplyReward>::from(single_row.columns.get(6).to_u32()),
        })
    }
}

