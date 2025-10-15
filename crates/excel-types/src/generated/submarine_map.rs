/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct SubmarineMap {
    pub row_id: u32,
    pub name: String,
    pub image_id: u32,
}

impl Sheet for SubmarineMap {
    const SHEET_NAME: &'static str = "SubmarineMap";
}

impl FromExcelRow for SubmarineMap {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            image_id: single_row.columns.get(1).to_u32(),
        })
    }
}

