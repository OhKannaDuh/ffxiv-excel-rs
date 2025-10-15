/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TextCommandParam {
    pub row_id: u32,
    pub param: String,
}

impl Sheet for TextCommandParam {
    const SHEET_NAME: &'static str = "TextCommandParam";
}

impl FromExcelRow for TextCommandParam {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            param: single_row.columns.get(0).to_owned_string(),
        })
    }
}

