/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HowTo {
    pub row_id: u32,
    pub name: String,
    pub announce: bool,
    pub category_id: u32,
    pub category: RowRef<HowToCategory>,
    pub sort: u8,
}

impl Sheet for HowTo {
    const SHEET_NAME: &'static str = "HowTo";
}

impl FromExcelRow for HowTo {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            announce: single_row.columns.get(1).to_bit(0),
            category_id: single_row.columns.get(12).to_u32(),
            category: RowRef::<HowToCategory>::from(single_row.columns.get(12).to_u32()),
            sort: single_row.columns.get(13).to_u8(),
        })
    }
}

