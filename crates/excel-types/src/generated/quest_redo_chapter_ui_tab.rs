/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct QuestRedoChapterUITab {
    pub row_id: u32,
    pub icon1_id: u32,
    pub icon2_id: u32,
    pub text: String,
}

impl Sheet for QuestRedoChapterUITab {
    const SHEET_NAME: &'static str = "QuestRedoChapterUITab";
}

impl FromExcelRow for QuestRedoChapterUITab {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            icon1_id: single_row.columns.get(1).to_u32(),
            icon2_id: single_row.columns.get(2).to_u32(),
            text: single_row.columns.get(3).to_owned_string(),
        })
    }
}

