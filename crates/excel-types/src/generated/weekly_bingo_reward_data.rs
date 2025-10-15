/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct WeeklyBingoRewardData {
    pub row_id: u32,
    pub reward_item_2_id: u32,
    pub reward_hq_2: bool,
    pub reward_quantity_2: u16,
}

impl Sheet for WeeklyBingoRewardData {
    const SHEET_NAME: &'static str = "WeeklyBingoRewardData";
}

impl FromExcelRow for WeeklyBingoRewardData {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            reward_item_2_id: single_row.columns.get(10).to_u32(),
            reward_hq_2: single_row.columns.get(11).to_bool(),
            reward_quantity_2: single_row.columns.get(12).to_u16(),
        })
    }
}

