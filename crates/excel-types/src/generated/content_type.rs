/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ContentType {
    pub row_id: u32,
    pub name: String,
    pub icon_id: u32,
    pub icon_duty_finder_id: u32,
}

impl Sheet for ContentType {
    const SHEET_NAME: &'static str = "ContentType";
}

impl FromExcelRow for ContentType {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            icon_id: single_row.columns.get(1).to_u32(),
            icon_duty_finder_id: single_row.columns.get(2).to_u32(),
        })
    }
}

