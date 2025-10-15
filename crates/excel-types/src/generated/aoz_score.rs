/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AOZScore {
    pub row_id: u32,
    pub is_hidden: bool,
    pub score: i32,
    pub name: String,
    pub description: String,
}

impl Sheet for AOZScore {
    const SHEET_NAME: &'static str = "AOZScore";
}

impl FromExcelRow for AOZScore {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            is_hidden: single_row.columns.get(0).to_bit(0),
            score: single_row.columns.get(1).to_i32(),
            name: single_row.columns.get(2).to_owned_string(),
            description: single_row.columns.get(3).to_owned_string(),
        })
    }
}

