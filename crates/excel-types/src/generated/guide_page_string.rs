/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GuidePageString {
    pub row_id: u32,
    pub string: String,
}

impl Sheet for GuidePageString {
    const SHEET_NAME: &'static str = "GuidePageString";
}

impl FromExcelRow for GuidePageString {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            string: single_row.columns.get(0).to_owned_string(),
        })
    }
}

