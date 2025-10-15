/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RecipeNotebookList {
    pub row_id: u32,
    pub count: u8,
}

impl Sheet for RecipeNotebookList {
    const SHEET_NAME: &'static str = "RecipeNotebookList";
}

impl FromExcelRow for RecipeNotebookList {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            count: single_row.columns.get(0).to_u8(),
        })
    }
}

