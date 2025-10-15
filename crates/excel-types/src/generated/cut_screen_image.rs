/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CutScreenImage {
    pub row_id: u32,
    pub _type: i16,
    pub image: i32,
}

impl Sheet for CutScreenImage {
    const SHEET_NAME: &'static str = "CutScreenImage";
}

impl FromExcelRow for CutScreenImage {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _type: single_row.columns.get(0).to_i16(),
            image: single_row.columns.get(1).to_i32(),
        })
    }
}

