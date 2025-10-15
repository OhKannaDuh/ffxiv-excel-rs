/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Tomestones {
    pub row_id: u32,
    pub weekly_limit: u16,
}

impl Sheet for Tomestones {
    const SHEET_NAME: &'static str = "Tomestones";
}

impl FromExcelRow for Tomestones {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            weekly_limit: single_row.columns.get(0).to_u16(),
        })
    }
}

