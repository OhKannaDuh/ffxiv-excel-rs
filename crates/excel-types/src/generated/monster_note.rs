/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MonsterNote {
    pub row_id: u32,
    pub reward: u32,
    pub name: String,
}

impl Sheet for MonsterNote {
    const SHEET_NAME: &'static str = "MonsterNote";
}

impl FromExcelRow for MonsterNote {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            reward: single_row.columns.get(8).to_u32(),
            name: single_row.columns.get(9).to_owned_string(),
        })
    }
}

