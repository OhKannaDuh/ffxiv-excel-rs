/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct LeveVfx {
    pub row_id: u32,
    pub effect: String,
    pub icon_id: u32,
}

impl Sheet for LeveVfx {
    const SHEET_NAME: &'static str = "LeveVfx";
}

impl FromExcelRow for LeveVfx {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            effect: single_row.columns.get(0).to_owned_string(),
            icon_id: single_row.columns.get(1).to_u32(),
        })
    }
}

