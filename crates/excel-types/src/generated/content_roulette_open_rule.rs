/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ContentRouletteOpenRule {
    pub row_id: u32,
    pub _type: u32,
}

impl Sheet for ContentRouletteOpenRule {
    const SHEET_NAME: &'static str = "ContentRouletteOpenRule";
}

impl FromExcelRow for ContentRouletteOpenRule {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _type: single_row.columns.get(1).to_u32(),
        })
    }
}

