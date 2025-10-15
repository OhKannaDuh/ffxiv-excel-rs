/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GimmickAccessor {
    pub row_id: u32,
    pub param0: i32,
    pub param1: u32,
    pub param2: u32,
    pub _type: u32,
}

impl Sheet for GimmickAccessor {
    const SHEET_NAME: &'static str = "GimmickAccessor";
}

impl FromExcelRow for GimmickAccessor {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            param0: single_row.columns.get(0).to_i32(),
            param1: single_row.columns.get(1).to_u32(),
            param2: single_row.columns.get(2).to_u32(),
            _type: single_row.columns.get(3).to_u32(),
        })
    }
}

