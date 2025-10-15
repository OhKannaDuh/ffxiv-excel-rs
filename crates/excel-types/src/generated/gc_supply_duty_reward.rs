/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GCSupplyDutyReward {
    pub row_id: u32,
    pub experience_supply: u32,
    pub experience_provisioning: u32,
    pub seals_expert_delivery: u32,
    pub seals_supply: u32,
    pub seals_provisioning: u32,
}

impl Sheet for GCSupplyDutyReward {
    const SHEET_NAME: &'static str = "GCSupplyDutyReward";
}

impl FromExcelRow for GCSupplyDutyReward {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            experience_supply: single_row.columns.get(0).to_u32(),
            experience_provisioning: single_row.columns.get(1).to_u32(),
            seals_expert_delivery: single_row.columns.get(2).to_u32(),
            seals_supply: single_row.columns.get(3).to_u32(),
            seals_provisioning: single_row.columns.get(4).to_u32(),
        })
    }
}

