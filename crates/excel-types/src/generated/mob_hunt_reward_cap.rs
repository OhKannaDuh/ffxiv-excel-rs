/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MobHuntRewardCap {
    pub row_id: u32,
    pub exp_cap: u32,
}

impl Sheet for MobHuntRewardCap {
    const SHEET_NAME: &'static str = "MobHuntRewardCap";
}

impl FromExcelRow for MobHuntRewardCap {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            exp_cap: single_row.columns.get(0).to_u32(),
        })
    }
}

