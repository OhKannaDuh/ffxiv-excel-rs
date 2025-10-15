/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct McGuffinUIData {
    pub row_id: u32,
    pub order: u16,
    pub icon_id: u32,
    pub name: String,
}

impl Sheet for McGuffinUIData {
    const SHEET_NAME: &'static str = "McGuffinUIData";
}

impl FromExcelRow for McGuffinUIData {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            order: single_row.columns.get(0).to_u16(),
            icon_id: single_row.columns.get(1).to_u32(),
            name: single_row.columns.get(2).to_owned_string(),
        })
    }
}

