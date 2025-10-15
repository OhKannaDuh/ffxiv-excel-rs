/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct PublicContentTextData {
    pub row_id: u32,
    pub text_data: String,
}

impl Sheet for PublicContentTextData {
    const SHEET_NAME: &'static str = "PublicContentTextData";
}

impl FromExcelRow for PublicContentTextData {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            text_data: single_row.columns.get(0).to_owned_string(),
        })
    }
}

