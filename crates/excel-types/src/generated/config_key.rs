/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ConfigKey {
    pub row_id: u32,
    pub label: String,
    pub param: u8,
    pub platform: u8,
    pub required: bool,
    pub category: u8,
    pub text: String,
}

impl Sheet for ConfigKey {
    const SHEET_NAME: &'static str = "ConfigKey";
}

impl FromExcelRow for ConfigKey {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            label: single_row.columns.get(0).to_owned_string(),
            param: single_row.columns.get(1).to_u8(),
            platform: single_row.columns.get(2).to_u8(),
            required: single_row.columns.get(3).to_bit(0),
            category: single_row.columns.get(4).to_u8(),
            text: single_row.columns.get(7).to_owned_string(),
        })
    }
}

