/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct WebURL {
    pub row_id: u32,
    pub url: String,
}

impl Sheet for WebURL {
    const SHEET_NAME: &'static str = "WebURL";
}

impl FromExcelRow for WebURL {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            url: single_row.columns.get(0).to_owned_string(),
        })
    }
}

