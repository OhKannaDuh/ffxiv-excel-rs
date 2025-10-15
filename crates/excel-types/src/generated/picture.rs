/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Picture {
    pub row_id: u32,
    pub image_id: u32,
    pub signature: i32,
}

impl Sheet for Picture {
    const SHEET_NAME: &'static str = "Picture";
}

impl FromExcelRow for Picture {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            image_id: single_row.columns.get(0).to_u32(),
            signature: single_row.columns.get(1).to_i32(),
        })
    }
}

