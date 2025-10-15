/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct LogMessage {
    pub row_id: u32,
    pub log_kind: u16,
    pub text: String,
}

impl Sheet for LogMessage {
    const SHEET_NAME: &'static str = "LogMessage";
}

impl FromExcelRow for LogMessage {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            log_kind: single_row.columns.get(0).to_u16(),
            text: single_row.columns.get(5).to_owned_string(),
        })
    }
}

