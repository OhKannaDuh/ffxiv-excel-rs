/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Completion {
    pub row_id: u32,
    pub group: u16,
    pub key: u16,
    pub lookup_table: String,
    pub text: String,
    pub group_title: String,
}

impl Sheet for Completion {
    const SHEET_NAME: &'static str = "Completion";
}

impl FromExcelRow for Completion {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            group: single_row.columns.get(0).to_u16(),
            key: single_row.columns.get(1).to_u16(),
            lookup_table: single_row.columns.get(2).to_owned_string(),
            text: single_row.columns.get(3).to_owned_string(),
            group_title: single_row.columns.get(4).to_owned_string(),
        })
    }
}

