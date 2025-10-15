/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct JournalGenre {
    pub row_id: u32,
    pub icon_id: u32,
    pub journal_category_id: u32,
    pub journal_category: RowRef<JournalCategory>,
    pub name: String,
}

impl Sheet for JournalGenre {
    const SHEET_NAME: &'static str = "JournalGenre";
}

impl FromExcelRow for JournalGenre {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            icon_id: single_row.columns.get(0).to_u32(),
            journal_category_id: single_row.columns.get(1).to_u32(),
            journal_category: RowRef::<JournalCategory>::from(single_row.columns.get(1).to_u32()),
            name: single_row.columns.get(3).to_owned_string(),
        })
    }
}

