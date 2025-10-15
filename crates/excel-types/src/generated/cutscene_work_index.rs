/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CutsceneWorkIndex {
    pub row_id: u32,
    pub work_index: u16,
}

impl Sheet for CutsceneWorkIndex {
    const SHEET_NAME: &'static str = "CutsceneWorkIndex";
}

impl FromExcelRow for CutsceneWorkIndex {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            work_index: single_row.columns.get(0).to_u16(),
        })
    }
}

