/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Omen {
    pub row_id: u32,
    pub path: String,
    pub path_ally: String,
    pub _type: u8,
    pub restrict_y_scale: bool,
    pub large_scale: bool,
}

impl Sheet for Omen {
    const SHEET_NAME: &'static str = "Omen";
}

impl FromExcelRow for Omen {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            path: single_row.columns.get(0).to_owned_string(),
            path_ally: single_row.columns.get(1).to_owned_string(),
            _type: single_row.columns.get(2).to_u8(),
            restrict_y_scale: single_row.columns.get(3).to_bit(0),
            large_scale: single_row.columns.get(4).to_bit(1),
        })
    }
}

