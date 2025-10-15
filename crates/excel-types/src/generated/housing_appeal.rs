/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HousingAppeal {
    pub row_id: u32,
    pub tag: String,
    pub icon_id: u32,
    pub order: u8,
}

impl Sheet for HousingAppeal {
    const SHEET_NAME: &'static str = "HousingAppeal";
}

impl FromExcelRow for HousingAppeal {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            tag: single_row.columns.get(0).to_owned_string(),
            icon_id: single_row.columns.get(1).to_u32(),
            order: single_row.columns.get(2).to_u8(),
        })
    }
}

