/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct PartyContentTextData {
    pub row_id: u32,
    pub data: String,
}

impl Sheet for PartyContentTextData {
    const SHEET_NAME: &'static str = "PartyContentTextData";
}

impl FromExcelRow for PartyContentTextData {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            data: single_row.columns.get(0).to_owned_string(),
        })
    }
}

