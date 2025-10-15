/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Tribe {
    pub row_id: u32,
    pub masculine: String,
    pub feminine: String,
    pub hp: i8,
    pub mp: i8,
    pub str: i8,
    pub vit: i8,
    pub dex: i8,
    pub int: i8,
    pub mnd: i8,
    pub pie: i8,
}

impl Sheet for Tribe {
    const SHEET_NAME: &'static str = "Tribe";
}

impl FromExcelRow for Tribe {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            masculine: single_row.columns.get(0).to_owned_string(),
            feminine: single_row.columns.get(1).to_owned_string(),
            hp: single_row.columns.get(2).to_i8(),
            mp: single_row.columns.get(3).to_i8(),
            str: single_row.columns.get(4).to_i8(),
            vit: single_row.columns.get(5).to_i8(),
            dex: single_row.columns.get(6).to_i8(),
            int: single_row.columns.get(7).to_i8(),
            mnd: single_row.columns.get(8).to_i8(),
            pie: single_row.columns.get(9).to_i8(),
        })
    }
}

