/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GuidePage {
    pub row_id: u32,
    pub key: u8,
    pub output_id: u32,
}

impl Sheet for GuidePage {
    const SHEET_NAME: &'static str = "GuidePage";
}

impl FromExcelRow for GuidePage {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            key: single_row.columns.get(0).to_u8(),
            output_id: single_row.columns.get(2).to_u32(),
        })
    }
}

