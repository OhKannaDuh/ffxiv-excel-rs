/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ModelState {
    pub row_id: u32,
    pub start_id: u32,
    pub start: RowRef<ActionTimeline>,
}

impl Sheet for ModelState {
    const SHEET_NAME: &'static str = "ModelState";
}

impl FromExcelRow for ModelState {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            start_id: single_row.columns.get(1).to_u32(),
            start: RowRef::<ActionTimeline>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

