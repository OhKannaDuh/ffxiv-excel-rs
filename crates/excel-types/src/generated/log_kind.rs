/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct LogKind {
    pub row_id: u32,
    pub format: bool,
}

impl Sheet for LogKind {
    const SHEET_NAME: &'static str = "LogKind";
}

impl FromExcelRow for LogKind {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            format: single_row.columns.get(1).to_bit(0),
        })
    }
}

