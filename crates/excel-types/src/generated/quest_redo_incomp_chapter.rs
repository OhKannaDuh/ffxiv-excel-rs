/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct QuestRedoIncompChapter {
    pub row_id: u32,
    pub chapter_id: u32,
}

impl Sheet for QuestRedoIncompChapter {
    const SHEET_NAME: &'static str = "QuestRedoIncompChapter";
}

impl FromExcelRow for QuestRedoIncompChapter {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            chapter_id: single_row.columns.get(0).to_u32(),
        })
    }
}

