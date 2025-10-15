/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct SkyIsland2RangeType {
    pub row_id: u32,
    pub _type: u8,
}

impl Sheet for SkyIsland2RangeType {
    const SHEET_NAME: &'static str = "SkyIsland2RangeType";
}

impl FromExcelRow for SkyIsland2RangeType {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _type: single_row.columns.get(0).to_u8(),
        })
    }
}

