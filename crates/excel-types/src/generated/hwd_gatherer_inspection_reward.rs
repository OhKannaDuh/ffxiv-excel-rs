/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HWDGathererInspectionReward {
    pub row_id: u32,
    pub scrips: u16,
    pub points: u16,
}

impl Sheet for HWDGathererInspectionReward {
    const SHEET_NAME: &'static str = "HWDGathererInspectionReward";
}

impl FromExcelRow for HWDGathererInspectionReward {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            scrips: single_row.columns.get(0).to_u16(),
            points: single_row.columns.get(1).to_u16(),
        })
    }
}

