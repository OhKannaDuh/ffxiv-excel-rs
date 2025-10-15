/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct LegacyQuest {
    pub row_id: u32,
    pub legacy_quest_id: u16,
    pub text: String,
    pub string: String,
    pub sort_key: u16,
    pub genre: u32,
}

impl Sheet for LegacyQuest {
    const SHEET_NAME: &'static str = "LegacyQuest";
}

impl FromExcelRow for LegacyQuest {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            legacy_quest_id: single_row.columns.get(0).to_u16(),
            text: single_row.columns.get(1).to_owned_string(),
            string: single_row.columns.get(2).to_owned_string(),
            sort_key: single_row.columns.get(3).to_u16(),
            genre: single_row.columns.get(4).to_u32(),
        })
    }
}

