/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Stain {
    pub row_id: u32,
    pub color_id: u32,
    pub shade: u8,
    pub sub_order: u8,
    pub name: String,
    pub name2: String,
}

impl Sheet for Stain {
    const SHEET_NAME: &'static str = "Stain";
}

impl FromExcelRow for Stain {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            color_id: single_row.columns.get(0).to_u32(),
            shade: single_row.columns.get(1).to_u8(),
            sub_order: single_row.columns.get(2).to_u8(),
            name: single_row.columns.get(3).to_owned_string(),
            name2: single_row.columns.get(4).to_owned_string(),
        })
    }
}

