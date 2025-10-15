/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct SatisfactionSupplyReward {
    pub row_id: u32,
    pub satisfaction_low: u16,
    pub satisfaction_mid: u16,
    pub satisfaction_high: u16,
    pub gil_low: u16,
    pub gil_mid: u16,
    pub gil_high: u16,
}

impl Sheet for SatisfactionSupplyReward {
    const SHEET_NAME: &'static str = "SatisfactionSupplyReward";
}

impl FromExcelRow for SatisfactionSupplyReward {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            satisfaction_low: single_row.columns.get(10).to_u16(),
            satisfaction_mid: single_row.columns.get(11).to_u16(),
            satisfaction_high: single_row.columns.get(12).to_u16(),
            gil_low: single_row.columns.get(13).to_u16(),
            gil_mid: single_row.columns.get(14).to_u16(),
            gil_high: single_row.columns.get(15).to_u16(),
        })
    }
}

