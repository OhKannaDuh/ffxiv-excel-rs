/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ContentCloseCycle {
    pub row_id: u32,
    pub unixtime: u32,
    pub time_seconds: u32,
}

impl Sheet for ContentCloseCycle {
    const SHEET_NAME: &'static str = "ContentCloseCycle";
}

impl FromExcelRow for ContentCloseCycle {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            unixtime: single_row.columns.get(0).to_u32(),
            time_seconds: single_row.columns.get(1).to_u32(),
        })
    }
}

