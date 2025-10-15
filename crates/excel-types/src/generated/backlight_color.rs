/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BacklightColor {
    pub row_id: u32,
    pub color_id: u32,
}

impl Sheet for BacklightColor {
    const SHEET_NAME: &'static str = "BacklightColor";
}

impl FromExcelRow for BacklightColor {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            color_id: single_row.columns.get(0).to_u32(),
        })
    }
}

