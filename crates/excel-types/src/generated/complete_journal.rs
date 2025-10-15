/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CompleteJournal {
    pub row_id: u32,
    pub required_level: u16,
    pub icon_id: u32,
    pub name: String,
}

impl Sheet for CompleteJournal {
    const SHEET_NAME: &'static str = "CompleteJournal";
}

impl FromExcelRow for CompleteJournal {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            required_level: single_row.columns.get(1).to_u16(),
            icon_id: single_row.columns.get(3).to_u32(),
            name: single_row.columns.get(5).to_owned_string(),
        })
    }
}

