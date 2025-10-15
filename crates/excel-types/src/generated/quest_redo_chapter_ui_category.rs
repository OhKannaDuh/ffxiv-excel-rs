/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct QuestRedoChapterUICategory {
    pub row_id: u32,
    pub expac: String,
}

impl Sheet for QuestRedoChapterUICategory {
    const SHEET_NAME: &'static str = "QuestRedoChapterUICategory";
}

impl FromExcelRow for QuestRedoChapterUICategory {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            expac: single_row.columns.get(1).to_owned_string(),
        })
    }
}

