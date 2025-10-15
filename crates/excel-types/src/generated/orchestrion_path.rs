/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct OrchestrionPath {
    pub row_id: u32,
    pub file: String,
}

impl Sheet for OrchestrionPath {
    const SHEET_NAME: &'static str = "OrchestrionPath";
}

impl FromExcelRow for OrchestrionPath {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            file: single_row.columns.get(0).to_owned_string(),
        })
    }
}

