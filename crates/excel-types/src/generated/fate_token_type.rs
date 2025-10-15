/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FateTokenType {
    pub row_id: u32,
    pub currency_id: u32,
    pub currency: RowRef<Item>,
}

impl Sheet for FateTokenType {
    const SHEET_NAME: &'static str = "FateTokenType";
}

impl FromExcelRow for FateTokenType {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            currency_id: single_row.columns.get(0).to_u32(),
            currency: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

