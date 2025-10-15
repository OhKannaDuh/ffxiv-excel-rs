/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJICraftworksSupplyDefine {
    pub row_id: u32,
    pub supply: i16,
    pub ratio: u16,
}

impl Sheet for MJICraftworksSupplyDefine {
    const SHEET_NAME: &'static str = "MJICraftworksSupplyDefine";
}

impl FromExcelRow for MJICraftworksSupplyDefine {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            supply: single_row.columns.get(0).to_i16(),
            ratio: single_row.columns.get(1).to_u16(),
        })
    }
}

