/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DeepDungeonMap5X {
    pub row_id: u32,
}

impl Sheet for DeepDungeonMap5X {
    const SHEET_NAME: &'static str = "DeepDungeonMap5X";
}

impl FromExcelRow for DeepDungeonMap5X {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
        })
    }
}

