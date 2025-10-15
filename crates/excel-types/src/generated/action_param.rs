/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ActionParam {
    pub row_id: u32,
    pub name: i16,
}

impl Sheet for ActionParam {
    const SHEET_NAME: &'static str = "ActionParam";
}

impl FromExcelRow for ActionParam {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_i16(),
        })
    }
}

