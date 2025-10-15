/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct JournalCategory {
    pub row_id: u32,
    pub name: String,
    pub separate_type: u8,
    pub data_type: u8,
    pub journal_section_id: u32,
    pub journal_section: RowRef<JournalSection>,
}

impl Sheet for JournalCategory {
    const SHEET_NAME: &'static str = "JournalCategory";
}

impl FromExcelRow for JournalCategory {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            separate_type: single_row.columns.get(1).to_u8(),
            data_type: single_row.columns.get(2).to_u8(),
            journal_section_id: single_row.columns.get(3).to_u32(),
            journal_section: RowRef::<JournalSection>::from(single_row.columns.get(3).to_u32()),
        })
    }
}

