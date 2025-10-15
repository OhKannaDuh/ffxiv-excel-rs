/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Condition {
    pub row_id: u32,
    pub log_message_id: u32,
    pub log_message: RowRef<LogMessage>,
}

impl Sheet for Condition {
    const SHEET_NAME: &'static str = "Condition";
}

impl FromExcelRow for Condition {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            log_message_id: single_row.columns.get(2).to_u32(),
            log_message: RowRef::<LogMessage>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

