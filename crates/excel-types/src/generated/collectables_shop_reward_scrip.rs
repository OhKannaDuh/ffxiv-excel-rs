/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CollectablesShopRewardScrip {
    pub row_id: u32,
    pub currency: u16,
    pub low_reward: u16,
    pub mid_reward: u16,
    pub high_reward: u16,
    pub exp_ratio_low: u16,
    pub exp_ratio_mid: u16,
    pub exp_ratio_high: u16,
}

impl Sheet for CollectablesShopRewardScrip {
    const SHEET_NAME: &'static str = "CollectablesShopRewardScrip";
}

impl FromExcelRow for CollectablesShopRewardScrip {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            currency: single_row.columns.get(0).to_u16(),
            low_reward: single_row.columns.get(1).to_u16(),
            mid_reward: single_row.columns.get(2).to_u16(),
            high_reward: single_row.columns.get(3).to_u16(),
            exp_ratio_low: single_row.columns.get(4).to_u16(),
            exp_ratio_mid: single_row.columns.get(5).to_u16(),
            exp_ratio_high: single_row.columns.get(6).to_u16(),
        })
    }
}

