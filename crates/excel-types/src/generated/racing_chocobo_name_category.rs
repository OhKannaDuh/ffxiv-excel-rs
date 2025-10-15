/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RacingChocoboNameCategory {
    pub row_id: u32,
    pub sort_key: u8,
    pub name: String,
}

impl Sheet for RacingChocoboNameCategory {
    const SHEET_NAME: &'static str = "RacingChocoboNameCategory";
}

impl FromExcelRow for RacingChocoboNameCategory {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            sort_key: single_row.columns.get(0).to_u8(),
            name: single_row.columns.get(1).to_owned_string(),
        })
    }
}

