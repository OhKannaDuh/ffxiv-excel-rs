/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AetheryteSystemDefine {
    pub row_id: u32,
    pub text: String,
    pub define_value: u32,
}

impl Sheet for AetheryteSystemDefine {
    const SHEET_NAME: &'static str = "AetheryteSystemDefine";
}

impl FromExcelRow for AetheryteSystemDefine {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            text: single_row.columns.get(0).to_owned_string(),
            define_value: single_row.columns.get(1).to_u32(),
        })
    }
}

