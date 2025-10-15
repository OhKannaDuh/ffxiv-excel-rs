/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct EventIconPriority {
    pub row_id: u32,
}

impl Sheet for EventIconPriority {
    const SHEET_NAME: &'static str = "EventIconPriority";
}

impl FromExcelRow for EventIconPriority {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
        })
    }
}

