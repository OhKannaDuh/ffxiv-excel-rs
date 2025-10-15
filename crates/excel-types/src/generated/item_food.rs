/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ItemFood {
    pub row_id: u32,
    pub exp_bonuspercent: u8,
}

impl Sheet for ItemFood {
    const SHEET_NAME: &'static str = "ItemFood";
}

impl FromExcelRow for ItemFood {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            exp_bonuspercent: single_row.columns.get(0).to_u8(),
        })
    }
}

