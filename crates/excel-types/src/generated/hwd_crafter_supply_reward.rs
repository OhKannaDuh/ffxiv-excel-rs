/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HWDCrafterSupplyReward {
    pub row_id: u32,
    pub script_reward_amount: u16,
    pub exp_reward: u32,
    pub points: u16,
}

impl Sheet for HWDCrafterSupplyReward {
    const SHEET_NAME: &'static str = "HWDCrafterSupplyReward";
}

impl FromExcelRow for HWDCrafterSupplyReward {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            script_reward_amount: single_row.columns.get(0).to_u16(),
            exp_reward: single_row.columns.get(1).to_u32(),
            points: single_row.columns.get(2).to_u16(),
        })
    }
}

