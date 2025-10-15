/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HugeCraftworksRank {
    pub row_id: u32,
    pub crafter_level: u8,
    pub exp_reward_per_item: u32,
}

impl Sheet for HugeCraftworksRank {
    const SHEET_NAME: &'static str = "HugeCraftworksRank";
}

impl FromExcelRow for HugeCraftworksRank {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            crafter_level: single_row.columns.get(0).to_u8(),
            exp_reward_per_item: single_row.columns.get(1).to_u32(),
        })
    }
}

