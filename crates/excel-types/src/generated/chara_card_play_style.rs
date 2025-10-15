/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CharaCardPlayStyle {
    pub row_id: u32,
    pub icon_id: u32,
    pub sort_key: u8,
    pub name: String,
}

impl Sheet for CharaCardPlayStyle {
    const SHEET_NAME: &'static str = "CharaCardPlayStyle";
}

impl FromExcelRow for CharaCardPlayStyle {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            icon_id: single_row.columns.get(0).to_u32(),
            sort_key: single_row.columns.get(1).to_u8(),
            name: single_row.columns.get(2).to_owned_string(),
        })
    }
}

