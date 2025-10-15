/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MobHuntReward {
    pub row_id: u32,
    pub exp_reward: u32,
    pub gil_reward: u16,
    pub expansion_id: u32,
    pub expansion: RowRef<ExVersion>,
    pub currency_reward: u16,
}

impl Sheet for MobHuntReward {
    const SHEET_NAME: &'static str = "MobHuntReward";
}

impl FromExcelRow for MobHuntReward {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            exp_reward: single_row.columns.get(0).to_u32(),
            gil_reward: single_row.columns.get(1).to_u16(),
            expansion_id: single_row.columns.get(2).to_u32(),
            expansion: RowRef::<ExVersion>::from(single_row.columns.get(2).to_u32()),
            currency_reward: single_row.columns.get(3).to_u16(),
        })
    }
}

