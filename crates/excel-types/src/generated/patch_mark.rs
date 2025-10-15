/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct PatchMark {
    pub row_id: u32,
    pub category: i8,
    pub sub_category_type: u8,
    pub sub_category: u16,
    pub mark_id: u32,
    pub version: u8,
}

impl Sheet for PatchMark {
    const SHEET_NAME: &'static str = "PatchMark";
}

impl FromExcelRow for PatchMark {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            category: single_row.columns.get(0).to_i8(),
            sub_category_type: single_row.columns.get(1).to_u8(),
            sub_category: single_row.columns.get(2).to_u16(),
            mark_id: single_row.columns.get(5).to_u32(),
            version: single_row.columns.get(6).to_u8(),
        })
    }
}

