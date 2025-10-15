/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJICraftworksRankRatio {
    pub row_id: u32,
    pub ratio: u16,
}

impl Sheet for MJICraftworksRankRatio {
    const SHEET_NAME: &'static str = "MJICraftworksRankRatio";
}

impl FromExcelRow for MJICraftworksRankRatio {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            ratio: single_row.columns.get(0).to_u16(),
        })
    }
}

