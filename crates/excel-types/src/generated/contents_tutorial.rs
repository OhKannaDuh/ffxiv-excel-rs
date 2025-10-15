/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ContentsTutorial {
    pub row_id: u32,
    pub name: u32,
    pub description: String,
}

impl Sheet for ContentsTutorial {
    const SHEET_NAME: &'static str = "ContentsTutorial";
}

impl FromExcelRow for ContentsTutorial {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(8).to_u32(),
            description: single_row.columns.get(9).to_owned_string(),
        })
    }
}

