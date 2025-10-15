/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct UDS_Property {
    pub row_id: u32,
    pub text: String,
    pub _type: String,
}

impl Sheet for UDS_Property {
    const SHEET_NAME: &'static str = "UDS_Property";
}

impl FromExcelRow for UDS_Property {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            text: single_row.columns.get(0).to_owned_string(),
            _type: single_row.columns.get(1).to_owned_string(),
        })
    }
}

