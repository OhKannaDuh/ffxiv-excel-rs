/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct PvPSelectTrait {
    pub row_id: u32,
    pub effect: String,
    pub icon_id: u32,
    pub value: i16,
}

impl Sheet for PvPSelectTrait {
    const SHEET_NAME: &'static str = "PvPSelectTrait";
}

impl FromExcelRow for PvPSelectTrait {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            effect: single_row.columns.get(0).to_owned_string(),
            icon_id: single_row.columns.get(1).to_u32(),
            value: single_row.columns.get(2).to_i16(),
        })
    }
}

