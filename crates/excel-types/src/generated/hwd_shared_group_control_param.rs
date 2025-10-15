/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HWDSharedGroupControlParam {
    pub row_id: u32,
    pub param_value: u8,
}

impl Sheet for HWDSharedGroupControlParam {
    const SHEET_NAME: &'static str = "HWDSharedGroupControlParam";
}

impl FromExcelRow for HWDSharedGroupControlParam {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            param_value: single_row.columns.get(1).to_u8(),
        })
    }
}

