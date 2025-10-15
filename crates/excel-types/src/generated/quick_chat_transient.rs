/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct QuickChatTransient {
    pub row_id: u32,
    pub text_output: String,
}

impl Sheet for QuickChatTransient {
    const SHEET_NAME: &'static str = "QuickChatTransient";
}

impl FromExcelRow for QuickChatTransient {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            text_output: single_row.columns.get(0).to_owned_string(),
        })
    }
}

