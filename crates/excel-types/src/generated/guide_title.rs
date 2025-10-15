/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GuideTitle {
    pub row_id: u32,
    pub title: String,
}

impl Sheet for GuideTitle {
    const SHEET_NAME: &'static str = "GuideTitle";
}

impl FromExcelRow for GuideTitle {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            title: single_row.columns.get(0).to_owned_string(),
        })
    }
}

