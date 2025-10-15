/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CircleActivity {
    pub row_id: u32,
    pub name: String,
    pub icon_id: u32,
    pub order: u16,
}

impl Sheet for CircleActivity {
    const SHEET_NAME: &'static str = "CircleActivity";
}

impl FromExcelRow for CircleActivity {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            icon_id: single_row.columns.get(1).to_u32(),
            order: single_row.columns.get(2).to_u16(),
        })
    }
}

